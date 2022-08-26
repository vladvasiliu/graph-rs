use crate::traits::ODataLink;

#[derive(Deserialize, Debug)]
pub struct ResponseBody<T> {
    value: Option<T>,
    #[serde(rename = "@odata.nextLink")]
    next_link: Option<String>,
    #[serde(rename = "@odata.deltaLink")]
    delta_link: Option<String>,
    #[serde(rename = "@odata.context")]
    metadata_link: Option<String>,
    #[serde(rename = "@microsoft.graph.downloadUrl")]
    download_link: Option<String>,
}

impl<T> ODataLink for ResponseBody<T> {
    fn download_link(&self) -> Option<String> {
        self.download_link.clone()
    }

    fn metadata_link(&self) -> Option<String> {
        self.metadata_link.clone()
    }

    fn next_link(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn delta_link(&self) -> Option<String> {
        self.delta_link.clone()
    }
}
