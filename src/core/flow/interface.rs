use std::path::Path;

use crate::core::flow::controller::interface::{exec_fl_flow, exec_toml_flow, exec_xml_flow};

pub async fn exec_flow(path: &Path) {
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap().to_lowercase().as_str() {
            // 目前首选支持flow/fl类的流程，其余都属于自定义的流类型
            // 目前其实flow也是json类型，但是后续flow可能会有加密之类的功能加上去
            "flow" | "fl" | "json" => exec_fl_flow(path).await,
            "xml" => exec_xml_flow(path),
            "toml" => exec_toml_flow(path),
            // 目前拒绝处理其他类型的流程
            _ => return,
        }
    } else {
        // 不解析其他任何后缀名的文件
        return;
    }
}
