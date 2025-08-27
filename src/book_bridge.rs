use pdfium::*;

/*
TODO
  create a struct that is very much handlable in
  any time so we need to make the struct refrence of pdf combination logic to json
  so it can be loaded and anytime we can load it
  back in any system
    folder strtucure is

         Pdfname/
            version (date+time as folder name)/
                                           /splited
                                           /failed
                                           /combined
                                           reCombine.json

           version (date+time as folder name)/
                                           /splited
                                           /failed
                                           /combined
                                           reCombine.json




*/

const MAX_PDF_SIZE:usize = 1024  * 1024 * 29;
const MAX_ALLOWED_IS_READABLE_TEXT:i32 = 20;
const SET_PAGE:i32 = 298;
const MAX_PAGE:i32 = SET_PAGE - 2;


pub fn split_it(path:String) -> String {
    let save_path:String="C:\\Users\\akash.v\\RustroverProjects\\untitled\\A17_FlightPlan.pdf".to_string();


    let pdf= load_pdf(&save_path);
    if let None = pdf {
        return "some thing went wrong".to_string();
    }

    /*
     pdf is loadded so initalize local
     variable to manage pdf page and size
    */
    let pdfium_document= pdf.unwrap();
    // Get number of pages
    let mut vec_i32 = vec![0i32; 1];
    let mut current_pdf_size:usize = 0;
    let mut current_pdf:usize = 0;
    let total_pages = pdfium_document.page_count();




    /*
      instance for failed
       here we can add all failed pdf files
    */
    let mut failed_pages_pdf = Pdf::new();
    let mut succespdf = Pdf::new();

    for i in 0..total_pages {
        vec_i32[0] = i;
        let size:usize = get_page_bytes(&pdfium_document,&vec_i32);
        let page_readable:bool = is_page_redable(&pdfium_document,i);

        if(page_readable || size > MAX_PDF_SIZE){
            let res=failed_pages_pdf.add_page(&pdfium_document, &vec_i32);
            match res {
                Some(data)=>{
                    println!("added to failed pdf index {} ",data );
                    },
                None =>{
                    println!("failed to add pdf" );
                }
            }
            continue;
        }

        println!("cccccc{} {}",succespdf.index,(current_pdf_size + size) <  MAX_PDF_SIZE);

        // 299 < 299 false  , 298 < 299 true so the pdf page will be 298 page

        if((current_pdf_size + size) >  MAX_PDF_SIZE || succespdf.index > MAX_PAGE){
            /*
              step 1
                 saving old succes pdf
             */
            {
                let path = format!(
                    "C:\\Users\\akash.v\\RustroverProjects\\untitled\\{}.pdf",
                    current_pdf
                );
                succespdf.pdf.save_to_path(path, None);
            }


            /*
             step 2
            clearing and assigning all back to 0
            so get accurate count for valid new pdf
            */
            current_pdf_size = 0;
            current_pdf += 1;
            succespdf = Pdf::new();

            let res=succespdf.add_page(&pdfium_document, &vec_i32);
            match res {
                Some(data)=>{
                    current_pdf_size += size;
                    println!("sucecsfully splited to  pdf index {} ",data );
                },
                None =>{
                    println!("failed to add pdf" );
                }
            }

        }else {

            let res=succespdf.add_page(&pdfium_document, &vec_i32);
            match res {
                Some(data)=>{
                    current_pdf_size += size;
                    println!("sucecsfully splited to  pdf index {} {} ",data ,current_pdf);
                },
                None =>{
                    println!("failed to add pdf" );
                }

            }
        }


        println!("page redable {} , page size is {}", page_readable, size);
    }

    //
    if(succespdf.index > 0){
        {
            let path = format!(
                "C:\\Users\\akash.v\\RustroverProjects\\untitled\\{}.pdf",
                current_pdf
            );
            succespdf.pdf.save_to_path(path, None);
        }
    }
    if(failed_pages_pdf.index > 0){
        failed_pages_pdf.pdf.save_to_path("C:\\Users\\akash.v\\RustroverProjects\\untitled\\failed.pdf",None);
    }else{
        println!("no error pdf found");
    }


    return "".to_string();
}


struct PagePackInfo{
    page_no:i32,
    file_no:u32,
    failed:bool,
}
struct BookBridge{
    path:String,
    version:String,
    total_page:u32,
    packed_info:Vec<PagePackInfo>
}


struct Pdf{
    pdf:PdfiumDocument,
    index:i32
}
impl Pdf {
    pub fn new() -> Pdf {
        Pdf{
            pdf:PdfiumDocument::new().unwrap(),
            index:-1
        }
    }

    pub fn add_page(&mut self, source_pdf: &PdfiumDocument, index_to_copy: &Vec<i32>) -> Option<i32> {
        let mut result = None;
        self.index += 1;
        
        let res=self.pdf.pages().import_by_index(source_pdf,Some(index_to_copy),self.index);

        match res {
            Result::Ok(data)=>{
                result = Some(self.index);
            },
            Result::Err(err)=>{
                self.index += 1;
            }
        }
        return result;
    }

}



fn get_page_bytes(source_pdf:&PdfiumDocument,index_to_copy:&Vec<i32>)->usize{
    let new_doc = PdfiumDocument::new().unwrap();
    new_doc.pages().by_ref().import_by_index(source_pdf,Some(&index_to_copy),0);
    //new_doc.save_to_path("C:\\Users\\akash.v\\RustroverProjects\\untitled\\ddd.pdf" ,None);
    let length:usize = new_doc.save_to_bytes(None).unwrap().len();
    return length;
}

fn is_page_redable(source_pdf:&PdfiumDocument,index:i32)-> bool {
    source_pdf.page(index).unwrap().text().unwrap().char_count().unwrap() > MAX_ALLOWED_IS_READABLE_TEXT
}


fn load_pdf(path:&String)-> Option<PdfiumDocument> {
    let pdfium = PdfiumDocument::new_from_path(path, None);
    let pdfium_document:Option<PdfiumDocument> = match pdfium {
        Ok(pdfium_document)=>{
            Some(pdfium_document)
        },
        Err(..)=>{
            println!("cannot load pdf");
            None
        }
    };
    return pdfium_document;
}



pub fn load_binary() {
    #[cfg(target_os = "windows")]
    {
        set_library_location("C:\\Users\\akash.v\\RustroverProjects\\untitled\\pdfium.dll");
    }

    #[cfg(target_os = "android")]
    {
        set_library_location("libpdfium.so");
    }

    #[cfg(target_os = "macos")]
    {
        // Example for macOS
        set_library_location("/usr/local/lib/libpdfium.dylib");
    }
}