use std::path::PathBuf;

use actix_web::App;
use failure::Error;
use tera::Tera;

use bindata::resolve;
use Bindata;
use Config;

pub struct OpenCTF {
    pub config: Config,
}

pub struct AppState {
    templates: Tera,
}

impl OpenCTF {
    pub fn new(config: Config) -> Result<Self, Error> {
        Ok(OpenCTF { config })
    }
    pub fn app(&self) -> Result<App<AppState>, Error> {
        let mut tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
        // load all templates
        let tmp: Bindata<String> = Bindata::new(PathBuf::from("templates"))?;
        let data = tmp.iter().map(|(k, v)| (k, resolve(v))).collect::<Vec<_>>();
        match tera.add_raw_templates(
            data.iter()
                .filter(|(_, v)| v.is_ok())
                .map(|(k, v)| (k, v.as_ref().unwrap()))
                .map(|(k, v)| (k.as_ref(), v.as_ref()))
                .collect(),
        ) {
            Err(err) => panic!("could not load templates: {}", err),
            _ => (),
        }

        let app = App::with_state(AppState { templates: tera });
        Ok(app)
    }
    pub fn bind_address(&self) -> (&str, u16) {
        (self.config.host.as_ref(), self.config.port)
    }
}
