use anyhow::{Context, Result};
use scraper::{Html, Selector};

const END_POINT: &str = "https://archriscv.felixc.at/.status/status.htm";

type BoxStr = Box<str>;

#[derive(Debug)]
pub struct PackageStatus {
    pub repo: BoxStr,
    pub pkgname: BoxStr,
    pub status: BoxStr,
}

impl PackageStatus {
    fn from(html_elems: &[&str]) -> PackageStatus {
        if html_elems.len() < 3 {
            panic!("unexpected html elems length, got elem: {:?}", html_elems)
        }

        Self {
            repo: html_elems[0].into(),
            pkgname: html_elems[1].into(),
            status: html_elems[2].into(),
        }
    }

    pub async fn download() -> Result<Vec<Self>> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Fail to create HTTP client to download status from felixc.at");

        let response = client
            .get(END_POINT)
            .send()
            .await
            .with_context(|| "fail to download status from felixc.at")?
            .text()
            .await
            .with_context(|| "fail to parse response into UTF-8 string")?;

        let fragment = Html::parse_fragment(&response);
        let selector = Selector::parse("tr").expect("invalid selector");
        let selected = fragment.select(&selector);

        let parsed = selected
            .into_iter()
            .filter(|elem| {
                // remove those package which is neither FTBFS or leaf
                let parts = elem.text().collect::<Vec<_>>();
                let status = parts[2];
                let is_ftbfs_pkg = status.contains("FTBFS");
                let is_leaf_pkg = status.contains("Leaf package");

                is_ftbfs_pkg || is_leaf_pkg
            })
            .map(|element| {
                let raw_str = element.text().collect::<Vec<_>>();
                Self::from(&raw_str)
            })
            .collect();

        Ok(parsed)
    }
}

#[tokio::test]
async fn test_parser() {
    println!("{:?}", PackageStatus::download().await.unwrap())
}
