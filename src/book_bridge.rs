use std::fs::{create_dir_all};
use std::path::PathBuf;
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

const MAX_PDF_SIZE:usize = 1024  * 1024 * 9;
const MAX_ALLOWED_IS_READABLE_TEXT:i32 = 20;
const SET_PAGE:i32 = 298;
const MAX_PAGE:i32 = SET_PAGE - 2;



pub fn split_it(source_path:String, save_location:String, mut closure: Box<dyn FnMut(i32)>) -> Option<String> {

    let processed_location = create_save_location(save_location);
    closure(1);
    if let Err(err) = processed_location {
        return Some(err);
    }

    let mut location =processed_location.unwrap();


    let pdf= load_pdf(&source_path);

    if let Err(err) = pdf {
        return Some(err);
    }
    closure(20);
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
    closure(30);



    /*
      instance for failed
       here we can add all failed pdf files
    */
    let mut failed_pages_pdf = Pdf::new();
    let mut succespdf = Pdf::new();

    closure(40);
    vec_i32[0] = 0;
    let result = get_page_bytes(&pdfium_document,&vec_i32);
    if let Err(err) = &result{
        return  Some(err.clone());
    }


    for i in 0..total_pages {
        vec_i32[0] = i;
        let result = get_page_bytes(&pdfium_document,&vec_i32);
        if let Err(err) = &result{
           return  Some(err.clone());
        }
        let size = result.unwrap();

        let page_readable:bool = is_page_redable(&pdfium_document,i);

        if(!page_readable || size > MAX_PDF_SIZE){
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
                    "{}.pdf",
                    current_pdf
                );
                &location.splited.push(path);

                println!("{}",location.splited.to_str().unwrap());
               let res=  succespdf.pdf.save_to_path(&location.splited, None);

                &location.splited.pop();
                println!("{}",location.splited.to_str().unwrap());
                if let Err(err) =res{
                    return  Some(err.to_string());
                }
            }


            /*
             step 2
            clearing and assigning all back to 0
            so get accurate count for valid new pdf
            */
            current_pdf_size = 0;
            current_pdf += 1;
            drop(succespdf);
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
    if(succespdf.index > -1){
        {

            let path = format!(
                "{}.pdf",
                current_pdf
            );
            &location.splited.push(path);

            let res=  succespdf.pdf.save_to_path(&location.splited, None);

            &location.splited.pop();
        }
    }
    if(failed_pages_pdf.index > -1){
        // let path = format!(
        //     "1.pdf",
        //     current_pdf
        // );
        &location.failed.push("1.pdf");
        failed_pages_pdf.pdf.save_to_path(&location.failed, None).expect("TODO: panic message");
        &location.failed.pop();
    }else{
        println!("no error pdf found");
    }

    closure(100);
    return Some("".to_string());
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
                self.index -= 1;
            }
        }
        return result;
    }

}


struct BookBridgePath{
    destinaton:PathBuf,
    splited:PathBuf,
    failed:PathBuf,
}

fn create_save_location(destination:String)-> Result<BookBridgePath,String> {
    let main_path=PathBuf::from(&destination);

    if !&main_path.is_dir() {
        if let Err(da) = create_dir_all(&main_path) {
            return  Err(format!("{} location is not valid",&main_path.to_str().unwrap()));
        }else {
            println!("{} location created",&main_path.to_str().unwrap());
        }
    }



    let mut failed=PathBuf::from(&destination);
    failed.push("failed");
    let mut splited=PathBuf::from(&destination);
    splited.push("splited");



    let creation=[splited,failed];


    for path in &creation {
        if !path.is_dir() {
            if let Err(da) = create_dir_all(&path) {
                println!("{} location is not valid",path.to_str().unwrap());
            }else {
                println!("{} location created",path.to_str().unwrap());
            }
        }
    }

    return Ok(BookBridgePath{
        failed:creation[1].clone(),
        destinaton:main_path,
        splited:creation[0].clone(),
    });
}



fn get_page_bytes(source_pdf:&PdfiumDocument,index_to_copy:&Vec<i32>)->Result<usize,String>{
    let res_new_doc_creation = PdfiumDocument::new();
    if let Err(err)= &res_new_doc_creation {
       return  Err(err.to_string());
    }
    let new_doc=res_new_doc_creation.unwrap();

    new_doc.pages().by_ref().import_by_index(source_pdf,Some(&index_to_copy),0);
    //new_doc.save_to_path("C:\\Users\\akash.v\\RustroverProjects\\untitled\\ddd.pdf" ,None);
    let length:usize = new_doc.save_to_bytes(None).unwrap().len();
    return Ok(length);
}

fn is_page_redable(source_pdf:&PdfiumDocument,index:i32)-> bool {
    source_pdf.page(index).unwrap().text().unwrap().char_count().unwrap() > MAX_ALLOWED_IS_READABLE_TEXT
}


fn load_pdf(path:&String)-> Result<PdfiumDocument, String> {
    let pdfium = PdfiumDocument::new_from_path(path, None);

    let pdfium_document = match  pdfium {
        Ok(pdfium_document)=>{
            Ok(pdfium_document)
        },
        Err(err)=>{
             Err(err.to_string())
        }
    };
    return pdfium_document;
}



pub fn load_binary(dynamic_path:&String) {
    set_library_location(dynamic_path);
}