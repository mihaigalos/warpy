use build_html::*;
use log::debug;
use std::fs::read_dir;
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use tokio::signal::ctrl_c;
use warp::filters::BoxedFilter;
use warp::path::FullPath;
use warp::reject::not_found;
use warp::reply::{html, Reply};
use warp::Filter;

pub async fn run(folder: String, ip: [u8; 4], port: u16) -> io::Result<()> {
    let ip_addr = IpAddr::from(ip);
    let socket_addr = SocketAddr::from((ip_addr, port));
    let handle = tokio::spawn(warp::serve(routes(folder)).bind(socket_addr));
    ctrl_c().await?;
    handle.abort();
    handle.await?;
    Ok(())
}

pub fn routes(folder: String) -> BoxedFilter<(impl Reply,)> {
    let logging = warp::log::custom(|info| {
        debug!("Request: '{}',\tStatus: '{}'", info.path(), info.status())
    });

    let handle_files = warp::fs::dir(folder.clone());
    let handle_directories = warp::get()
        .and(warp::path::full())
        .and_then(move |route| path_to_html(folder.clone(), route))
        .map(html);

    handle_files.or(handle_directories).with(logging).boxed()
}

async fn path_to_html(folder: String, route: FullPath) -> Result<String, warp::reject::Rejection> {
    let path = PathBuf::from(folder.clone()).join(&route.as_str()[1..]);

    let content = HtmlPage::new()
        .with_style(include_str!("styles.css"))
        .with_container(
            Container::new(ContainerType::Main)
                .with_attributes([("class", "border-box")])
                .with_preformatted_attr(route.as_str(), [("id", "header")])
                .with_container(
                    links_container(folder, path.as_path(), &route).ok_or_else(not_found)?,
                ),
        )
        .to_html_string();

    Ok(content)
}

fn links_container(folder: String, path: &Path, route: &FullPath) -> Option<Container> {
    let content_attrs = [("class", "content")];
    let mut links = Container::new(ContainerType::Div).with_attributes([("id", "wrapper")]);

    if route.as_str() != "/" {
        let parent = path
            .parent()
            .and_then(|path| path.strip_prefix(folder.clone()).ok())
            .and_then(Path::to_str)
            .map(|s| format!("/{}", s))?;
        links.add_link_attr(parent, "..", content_attrs);
    }

    let links_with_files = mixin_files(folder.clone(), path, links);
    let result = mixin_folders(folder, path, links_with_files.unwrap());
    result
}

fn mixin_files(folder: String, path: &Path, mut links: Container) -> Option<Container> {
    let content_attrs = [("class", "content")];
    let mut entries: Vec<(String, String, &'static str)> = read_dir(&path)
        .ok()?
        .filter_map(|res| res.ok().map(|x| x.path()))
        .filter_map(|path| format_path_folder(folder.clone(), path))
        .collect();
    entries.sort_by_cached_key(|(_, name, _)| name.to_string());
    for (path, name, icon) in entries {
        let link_text = format!("{}<p class=\"text\">{}</p>", icon, name);
        links.add_link_attr(path, link_text, content_attrs);
    }
    Some(links)
}

fn mixin_folders(folder: String, path: &Path, mut links: Container) -> Option<Container> {
    let content_attrs = [("class", "content")];
    let mut entries: Vec<(String, String, &'static str)> = read_dir(&path)
        .ok()?
        .filter_map(|res| res.ok().map(|x| x.path()))
        .filter_map(|path| format_path_file(folder.clone(), path))
        .collect();
    entries.sort_by_cached_key(|(_, name, _)| name.to_string());
    for (path, name, icon) in entries {
        let link_text = format!("{}<p class=\"text\">{}</p>", icon, name);
        links.add_link_attr(path, link_text, content_attrs);
    }

    Some(links)
}

fn format_path_file(folder: String, path: PathBuf) -> Option<(String, String, &'static str)> {
    let net_path = format!("/{}", path.strip_prefix(folder).ok()?.to_str()?);
    let file_name = path.file_name()?.to_str()?.into();
    if !path.is_dir() {
        return Some((net_path, file_name, "📄"));
    }
    None
}

fn format_path_folder(folder: String, path: PathBuf) -> Option<(String, String, &'static str)> {
    let net_path = format!("/{}", path.strip_prefix(folder).ok()?.to_str()?);
    let file_name = path.file_name()?.to_str()?.into();
    if path.is_dir() {
        return Some((net_path, file_name, "📁"));
    }
    None
}
