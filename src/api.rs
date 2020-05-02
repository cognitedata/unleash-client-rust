/// https://unleash.github.io/docs/api/client/features
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Features {
    pub version: u8,
    pub features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub strategies: Vec<Strategy>,
    pub variants: Option<Vec<Variant>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Strategy {
    pub name: String,
    pub parameters: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    pub name: String,
    pub weight: u8,
    pub payload: Option<HashMap<String, String>>,
    pub overrides: Option<Vec<VariantOverride>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct VariantOverride {
    #[serde(rename = "contextName")]
    pub context_name: String,
    pub values: Vec<String>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_reference_doc() -> Result<(), serde_json::Error> {
        let data = r#"
        {
            "version": 1,
            "features": [
              {
                "name": "F1",
                "description": "Default Strategy, enabledoff, variants",
                "enabled": false,
                "strategies": [
                  {
                    "name": "default"
                  }
                ],
                "variants":[
                    {"name":"Foo","weight":50,"payload":{"type":"string","value":"bar"}},
                    {"name":"Bar","weight":50,"overrides":[{"contextName":"userId","values":["robert"]}]}
                    ],
                "createdAt": "2020-04-28T07:26:27.366Z"
              },
              {
                "name": "F2",
                "description": "customStrategy+params, enabled",
                "enabled": true,
                "strategies": [
                  {
                    "name": "customStrategy",
                    "parameters": {
                      "strategyParameter": "data,goes,here"
                    }
                  }
                ],
                "variants": null,
                "createdAt": "2020-01-12T15:05:11.462Z"
              },
              {
                "name": "F3",
                "description": "two strategies",
                "enabled": true,
                "strategies": [
                  {
                    "name": "customStrategy",
                    "parameters": {
                      "strategyParameter": "data,goes,here"
                    }
                  },
                  {
                    "name": "default",
                    "parameters": {}
                  }
                ],
                "variants": null,
                "createdAt": "2019-09-30T09:00:39.282Z"
              },
              {
                "name": "F4",
                "description": "Multiple params",
                "enabled": true,
                "strategies": [
                  {
                    "name": "customStrategy",
                    "parameters": {
                      "p1": "foo",
                      "p2": "bar"
                    }
                  }
                ],
                "variants": null,
                "createdAt": "2020-03-17T01:07:25.713Z"
              }
            ]
          }
        "#;
        let parsed: super::Features = serde_json::from_str(data)?;
        assert_eq!(1, parsed.version);
        Ok(())
    }
}