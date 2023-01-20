use crate::types::Response;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(typescript_custom_section)]
const CERTIFICATION_RESULT_TYPE: &'static str = r#"
interface CertificationResult {
  passed: boolean;
  response: Response;
}
"#;

#[derive(Debug, Eq, PartialEq)]
pub struct CertificationResult {
    pub passed: bool,
    pub response: Option<Response>,
}

#[cfg(target_arch = "wasm32")]
impl From<CertificationResult> for JsValue {
    fn from(certification_result: CertificationResult) -> Self {
        use js_sys::{Array, Boolean, Object};

        let passed = Boolean::from(certification_result.passed);
        let response = JsValue::from(certification_result.response);

        let passed_entry = Array::of2(&JsValue::from("passed"), &passed.into());
        let response_entry = Array::of2(&JsValue::from("response"), &response.into());

        let result = Object::from_entries(&Array::of2(&passed_entry, &response_entry)).unwrap();

        JsValue::from(result)
    }
}

#[cfg(all(target_arch = "wasm32", test))]
mod tests {
    use crate::types::{CertificationResult, Response};
    use js_sys::JSON;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn serialize_certification_result_with_no_response() {
        let expected = r#"{"passed":true}"#;

        assert_eq!(
            JSON::stringify(&JsValue::from(CertificationResult {
                passed: true,
                response: None
            }))
            .unwrap(),
            expected
        );
    }

    #[wasm_bindgen_test]
    fn serialize_certification_result_with_response() {
        let expected = r#"{"passed":true,"response":{"body":{"0":0,"1":1,"2":2},"headers":[]}}"#;

        assert_eq!(
            JSON::stringify(&JsValue::from(CertificationResult {
                passed: true,
                response: Some(Response {
                    body: vec![0, 1, 2],
                    headers: vec![],
                })
            }))
            .unwrap(),
            expected
        );
    }
}