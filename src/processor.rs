use crate::options::CmdOptions;
use image::io::Reader as ImageReader;
use image::DynamicImage as DynamicImage;

pub struct Processor {
    pub options: CmdOptions,
    img: DynamicImage,
}

impl Processor {
    pub fn new(options : CmdOptions) -> Self {
        let file = options.input_img.clone();
        Processor { options: options, img: ImageReader::open(file).unwrap().decode().unwrap() }
    }

    pub fn process_image(self) {
        
    }
}


// impl Processor {

// }

