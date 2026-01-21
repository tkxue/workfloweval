use super::*;

impl XdomA_Css_Util {
    const s: &'static str = r#"

div {
  user-select: none;
}

.container {
  display: flex;
  justify-content: center;
  align-items: center;
}

.display_none {
  display: none;
}

.visibility_hidden {
  visibility: hidden;
  display: none;
  width: 0px;
  height: 0px;
}

.visibility_normal {
  width: 100%;
  height: 100%;
}

#main_css_div {
  width: 100%;
  height: 100%;
}

body {
  user-select: none;
  border: 0;
  margin: 0;
  padding: 0;
  overflow: hidden;
  font-size: 24;
  font-family: monospace;
  font-weight: 200;
}

div, textarea, button {
  font-size: 20;
  font-family: monospace;
  font-weight: 200;
}

pre {
  font-size: 20;
  font-family: monospace;
  font-weight: 200;
  border: 0;
  margin: 0;
  padding: 0;
}


    "#;

    pub fn full_css() -> &'static str {
        &Self::s
    }
}

pub struct XdomA_Css_Util {}
