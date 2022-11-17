mod structures;

use crate::structures::*;

use actix_web::{HttpServer, App, web, Responder};



//use std::borrow::Borrow;
use std::collections::HashMap;
//use core::num::dec2flt::parse;
use std::io;

//use actix_web::Responder;
use nom::{
 
   branch::alt,
   bytes::complete::{tag, take_while},
   error::{ErrorKind, ParseError},
   multi::separated_list0,
   IResult,
};

use nom_locate::LocatedSpan;
type Span<'a> = LocatedSpan<&'a str>;



const HL7V2_VERSIONS: &[&str] = &[
       "2.1", "2.2", "2.3", "2.3.1", "2.4", "2.5", "2.6.1", "2.7", "2.7.1", "2.8",
];



// pub struct Msg {
// pub segments: Vec<Segment>,
// pub separator: char,
// }



impl Msg {

     pub fn slen(&self) -> usize { self.segments.len() }

     pub fn msg_type(&self) -> Option<String> {
 
         if let Some(segment) = self.segments.first() {


         if let Some(field) = segment.fields.get(8) {
             // If 3 components, take last (msg structure) //ADT^A01^ADT_A01
             // else take first and second component //ADT^A01 //firstcomp^secondcomp
            if field.components.len() == 3 && !field.components.last().unwrap().is_empty()
               {
                        return Some(field.components.last().unwrap().to_string());


               } else { 

                   //ADT^A01


                     let mut msg_type = "".to_owned();
                     if let Some(component) = field.components.first() {
                         // TODO: Check if it is valid type
                         msg_type.push_str(&component);
                         msg_type.push('_');
 
                         if let Some(component) = field.components.get(1) {
 
                            
                            msg_type.push_str(&component);
                            return Some(msg_type);
                                           
                            }           
                        }
                }
            }
        }
 None
 }

    pub fn version(&self) -> Option<String> {


         if let Some(segment) = self.segments.first() {
 
 
            if let Some(field) = segment.fields.get(11) {
               if let Some(component) = field.components.first() {
 
                   if HL7V2_VERSIONS.contains(&component.as_str()) {
 
                      let mut version = "V".to_owned();
                      version.push_str(component.as_str().replace(".", "_").as_str());
                      return Some(version);
 
 
                    }
                }
            }
         }
        None
     }

}




fn is_not_cs_fs_or_line_end(i: char) -> bool { i != '^' && i != '|' && i != '\n' && i != '\r' }






fn parse_component(i: Span) -> IResult<Span, Span> { 
 
 
 take_while(is_not_cs_fs_or_line_end)(i) 



}








fn parse_field(i: Span) -> IResult<Span, Field> {



 separated_list0( tag("^") , parse_component)(i).map(|(i, components)| {
          (i, Field {
                     components: components
                     .iter()
                     .map(|&s| {
                     s.fragment().to_string()
 
                     })
                     .collect(),
                    })
            })
}
















fn parse_segment(i: Span) -> IResult<Span, Segment> {
     
     separated_list0(tag("|"), parse_field)(i).map(|(i, fields)| {
           (i, Segment {
               fields: fields
               .into_iter()
               .map(|field| {
               field
 
               })
               .collect(),
            })
        })
}












pub fn parse_msg(i: Span) -> IResult<Span, Msg> {
 

     let separator = match i.chars().nth(3) {
         Some(c) => {
             if c != '|' {
                return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                 ErrorKind::Char,
                )));
            }
        c
        },
        None => {

               return Err(nom::Err::Error(ParseError::from_error_kind(
               i,
      ErrorKind::Eof,
                )));
            },
        };

    separated_list0(alt((tag("\n"), tag("\r"))), parse_segment)(i).map(|(i, segments)| {
             (i, Msg {
                segments: segments
                .into_iter()
                .map(|segment| {
                     segment
                    })
                    .collect(),
                    separator,
                })
        })
}





async fn status() -> impl Responder{
      let defaultversion_27adt = "MSH|^~\\&|EPIC||SMS|SMSDT|201501011408||ADT^A04|9000123|D|2.7|\nEVN|A04|201501011408\nPID||0493575^^^2^ID 1|454721||DOE^JOHN\nNK1||CONROY^MARI|SPO||(216)731-4359\nNK1||DOE^JOHNNY^^^^|CHD||(216)731-4222\nNK1||DOE^ROBERT ^^^^|CHD||(216)731-4222";
      //let real = "MSH|^~\\&|EPIC|EPICADT|SMS|SMSADT|199912271408|CHARRIS|ADT^A04|1817457|D|2.7||||||||||||||\nPID||0493575^^^2^ID 1|454721||DOE^JOHN^^^^|DOE^JOHN^^^^|19480203|M||B|254 MYSTREET AVE^^MYTOWN^OH^44123^USA||(216)123-4567|||M|NON|400003403~1129086||||||||||||||||||||||||\nNK1||ROE^MARIE^^^^|SPO||(216)123-4567||EC|||||||||||||||||||||||||||||||||||";
      let (_, msg) = parse_msg(Span::new(defaultversion_27adt)).unwrap();
      //let version = msg.version().unwrap();
      let msgtype = msg.msg_type().unwrap(); 

 

      let mut adt_database:HashMap<String,Vec<&str>> = HashMap::new(); 
      
      let msh = vec!["Field Separator","Encoding Characters","Sending Application","Sending Facility","Receiving Application","Receiving Facility","Date/Time Of Message","Security","Message Type","Message Control Id","Processing Id","Version Id","Sequence Number","Continuation Pointer","Accept Acknowledgment Type","Application Acknowledgment Type","Country Code","Character Set","Principal Language Of Message","Alternate Character Set Handling Scheme","Message Profile Identifier","Sending Responsible Organization","Receiving Responsible Organization","Sending Network Address","Receiving Network Address"];
      let sft = vec!["Software Vendor Organization","Software Certified Version Or Release Number","Software Product Name","Software Binary Id","Software Product Information","Software Install Date"];
      let uac = vec!["User Authentication Credential Type Code","User Authentication Credential"];
      let evn = vec!["Event Type Code","Recorded Date/Time","Date/Time Planned Event","Event Reason Code","Operator Id","Event Occurred","Event Facility"];
      let pid = vec!["Set Id - Pid","Patient Id","Patient Identifier List","Alternate Patient Id - Pid","Patient Name","Mother's Maiden Name","Date/Time Of Birth","Administrative Sex","Patient Alias","Race","Patient Address","County Code","Phone Number - Home","Phone Number - Business","Primary Language","Marital Status","Religion","Patient Account Number","Social Security Number - Patient","Driver's License Number - Patient","Mother's Identifier","Ethnic Group","Birth Place","Multiple Birth Indicator","Birth Order","Citizenship","Veterans Military Status","Nationality","Patient Death Date And Time","Patient Death Indicator","Identity Unknown Indicator","Identity Reliability Code","Last Update Date/Time","Last Update Facility","Species Code","Breed Code","Strain","Production Class Code","Tribal Citizenship","Patient Telecommunication Information"];
      let pd1 = vec!["Living Dependency","Living Arrangement","Patient Primary Facility","Patient Primary Care Provider Name & Id No.","Student Indicator","Handicap","Living Will Code","Organ Donor Code","Separate Bill","Duplicate Patient","Publicity Code","Protection Indicator","Protection Indicator Effective Date","Place Of Worship","Advance Directive Code","Immunization Registry Status","Immunization Registry Status Effective Date","Publicity Code Effective Date","Military Branch","Military Rank/Grade","Military Status","Advance Directive Last Verified Date"];
      let arv = vec!["Set Id","Access Restriction Action Code","Access Restriction Value","Access Restriction Reason","Special Access Restriction Instructions","Access Restriction Date Range"];
      let rol = vec!["Role Instance Id","Action Code","Role-rol","Role Person","Role Begin Date/Time","Role End Date/Time","Role Duration","Role Action Reason","Provider Type","Organization Unit Type","Office/Home Address/Birthplace","Phone","Person's Location","Organization"];
      let nk1 = vec!["Set Id","Name","Relationship","Address","Phone Number","Business Phone Number","Contact Role","Start Date","End Date","Next Of Kin / Associated Parties Job Title","Next Of Kin / Associated Parties Job Code/Class","Next Of Kin / Associated Parties Employee Number","Organization Name-NK1","Marital Status","Administrative Sex","Date/Time Of Birth","Living Dependency","Ambulatory Status","Citizenship","Primary Language","Living Arrangement","Publicity Code","Protection Indicator","Student Indicator","Religion","Mother's Maiden Name","Nationality","Ethnic Group","Contact Reason","Contact Person's Name","Contact Person's Telephone Number","Contact Person's Address","Next Of Kin/Associated Party's Identifiers","Job Status","Race","Handicap","Contact Person Social Security Number","Next Of Kin Birth Place","Vip Indicator","Next Of Kin Telecommunication Information","Contact Person's Telecommunication Information"];
      let pv1 = vec!["Set Id - Pv1","Patient Class","Assigned Patient Location","Admission Type","Preadmit Number","Prior Patient Location","Attending Doctor","Referring Doctor","Consulting Doctor","Hospital Service","Temporary Location","Preadmit Test Indicator","Re-admission Indicator","Admit Source","Ambulatory Status","Vip Indicator","Admitting Doctor","Patient Type","Visit Number","Financial Class","Charge Price Indicator","Courtesy Code","Credit Rating","Contract Code","Contract Effective Date","Contract Amount","Contract Period","Interest Code","Transfer To Bad Debt Code","Transfer To Bad Debt Date","Bad Debt Agency Code","Bad Debt Transfer Amount","Bad Debt Recovery Amount","Delete Account Indicator","Delete Account Date","Discharge Disposition","Discharged To Location","Diet Type","Servicing Facility","Bed Status","Account Status","Pending Location","Prior Temporary Location","Admit Date/Time","Discharge Date/Time","Current Patient Balance","Total Charges","Total Adjustments","Total Payments","Alternate Visit Id","Visit Indicator","Other Healthcare Provider","Service Episode Description","Service Episode Identifier"];
      let pv2 = vec!["Prior Pending Location","Accommodation Code","Admit Reason","Transfer Reason","Patient Valuables","Patient Valuables Location","Visit User Code","Expected Admit Date/Time ","Expected Discharge Date/Time ","Estimated Length Of Inpatient Stay","Actual Length Of Inpatient Stay","Visit Description","Referral Source Code","Previous Service Date","Employment Illness Related Indicator","Purge Status Code","Purge Status Date","Special Program Code","Retention Indicator","Expected Number Of Insurance Plans","Visit Publicity Code","Visit Protection Indicator","Clinic Organization Name","Patient Status Code","Visit Priority Code","Previous Treatment Date ","Expected Discharge Disposition","Signature On File Date","First Similar Illness Date ","Patient Charge Adjustment Code","Recurring Service Code","Billing Media Code","Expected Surgery Date And Time","Military Partnership Code","Military Non-availability Code","Newborn Baby Indicator","Baby Detained Indicator","Mode Of Arrival Code","Recreational Drug Use Code","Admission Level Of Care Code","Precaution Code","Patient Condition Code","Living Will Code","Organ Donor Code","Advance Directive Code","Patient Status Effective Date","Expected Loa Return Date/Time","Expected Pre-admission Testing Date/Time","Notify Clergy Code","Advance Directive Last Verified Date"];
      let db1 = vec!["Set Id - Db1","Disabled Person Code","Disabled Person Identifier","Disability Indicator","Disability Start Date","Disability End Date","Disability Return To Work Date","Disability Unable To Work Date"];
      let obx = vec!["Set Id - Obx","Value Type","Observation Identifier","Observation Sub-id","Observation Value","Units","References Range","Interpretation Codes","Probability","Nature Of Abnormal Test","Observation Result Status","Effective Date Of Reference Range","User Defined Access Checks","Date/Time Of The Observation","Producer's Id","Responsible Observer","Observation Method","Equipment Instance Identifier","Date/Time Of The Analysis","Observation Site","Observation Instance Identifier","Mood Code","Performing Organization Name","Performing Organization Address","Performing Organization Medical Director","Patient Results Release Category"];
      let al1 = vec!["Set Id - Al1","Allergen Type Code","Allergen Code/Mnemonic/Description","Allergy Severity Code","Allergy Reaction Code","Identification Date"];
      let dg1 = vec!["Set Id - Dg1","Diagnosis Coding Method","Diagnosis Code - Dg1","Diagnosis Description","Diagnosis Date/Time","Diagnosis Type","Major Diagnostic Category","Diagnostic Related Group","Drg Approval Indicator","Drg Grouper Review Code","Outlier Type","Outlier Days","Outlier Cost","Grouper Version And Type","Diagnosis Priority","Diagnosing Clinician","Diagnosis Classification","Confidential Indicator","Attestation Date/Time","Diagnosis Identifier","Diagnosis Action Code","Parent Diagnosis","Drg Ccl Value Code","Drg Grouping Usage","Drg Diagnosis Determination Status","Present On Admission (poa) Indicator"];
      let drg = vec!["Diagnostic Related Group","Drg Assigned Date/Time","Drg Approval Indicator","Drg Grouper Review Code","Outlier Type","Outlier Days","Outlier Cost","Drg Payor","Outlier Reimbursement","Confidential Indicator","Drg Transfer Type","Name Of Coder","Grouper Status","Pccl Value Code","Effective Weight","Monetary Amount","Status Patient","Grouper Software Name","Grouper Software Version","Status Financial Calculation","Relative Discount/Surcharge","Basic Charge","Total Charge","Discount/Surcharge","Calculated Days","Status Gender","Status Age","Status Length Of Stay","Status Same Day Flag","Status Separation Mode","Status Weight At Birth","Status Respiration Minutes","Status Admission"];
      let pr1 = vec!["Set Id - Pr1","Procedure Coding Method","Procedure Code","Procedure Description","Procedure Date/Time","Procedure Functional Type","Procedure Minutes","Anesthesiologist","Anesthesia Code","Anesthesia Minutes","Surgeon","Procedure Practitioner","Consent Code","Procedure Priority","Associated Diagnosis Code","Procedure Code Modifier","Procedure Drg Type","Tissue Type Code","Procedure Identifier","Procedure Action Code","Drg Procedure Determination Status","Drg Procedure Relevance","Treating Organizational Unit","Respiratory Within Surge"];
      let gt1 = vec!["Set Id - Gt1","Guarantor Number","Guarantor Name","Guarantor Spouse Name","Guarantor Address","Guarantor Ph Num - Home","Guarantor Ph Num - Business","Guarantor Date/Time Of Birth","Guarantor Administrative Sex","Guarantor Type","Guarantor Relationship","Guarantor Ssn","Guarantor Date - Begin","Guarantor Date - End","Guarantor Priority","Guarantor Employer Name","Guarantor Employer Address","Guarantor Employer Phone Number","Guarantor Employee Id Number","Guarantor Employment Status","Guarantor Organization Name","Guarantor Billing Hold Flag","Guarantor Credit Rating Code","Guarantor Death Date And Time","Guarantor Death Flag","Guarantor Charge Adjustment Code","Guarantor Household Annual Income","Guarantor Household Size","Guarantor Employer Id Number","Guarantor Marital Status Code","Guarantor Hire Effective Date","Employment Stop Date","Living Dependency","Ambulatory Status","Citizenship","Primary Language","Living Arrangement","Publicity Code","Protection Indicator","Student Indicator","Religion","Mother's Maiden Name","Nationality","Ethnic Group","Contact Person's Name","Contact Person's Telephone Number","Contact Reason","Contact Relationship","Job Title","Job Code/Class","Guarantor Employer's Organization Name","Handicap","Job Status","Guarantor Financial Class","Guarantor Race","Guarantor Birth Place","Vip Indicator"];
      let in1 = vec!["Set Id - In1","Health Plan Id","Insurance Company Id","Insurance Company Name","Insurance Company Address","Insurance Co Contact Person","Insurance Co Phone Number","Group Number","Group Name","Insured's Group Emp Id","Insured's Group Emp Name","Plan Effective Date","Plan Expiration Date","Authorization Information","Plan Type","Name Of Insured","Insured's Relationship To Patient","Insured's Date Of Birth","Insured's Address","Assignment Of Benefits","Coordination Of Benefits","Coord Of Ben. Priority","Notice Of Admission Flag","Notice Of Admission Date","Report Of Eligibility Flag","Report Of Eligibility Date","Release Information Code","Pre-admit Cert (pac)","Verification Date/Time","Verification By","Type Of Agreement Code","Billing Status","Lifetime Reserve Days","Delay Before L.r. Day","Company Plan Code","Policy Number","Policy Deductible","Policy Limit - Amount","Policy Limit - Days","Room Rate - Semi-private","Room Rate - Private","Insured's Employment Status","Insured's Administrative Sex","Insured's Employer's Address","Verification Status","Prior Insurance Plan Id","Coverage Type","Handicap","Insured's Id Number","Signature Code","Signature Code Date","Insured's Birth Place","Vip Indicator","External Health Plan Identifiers"];
      let in2 = vec!["Insured's Employee Id","Insured's Social Security Number","Insured's Employer's Name And Id","Employer Information Data","Mail Claim Party","Medicare Health Ins Card Number","Medicaid Case Name","Medicaid Case Number","Military Sponsor Name","Military Id Number","Dependent Of Military Recipient","Military Organization","Military Station","Military Service","Military Rank/Grade","Military Status","Military Retire Date","Military Non-avail Cert On File","Baby Coverage","Combine Baby Bill","Blood Deductible","Special Coverage Approval Name","Special Coverage Approval Title","Non-covered Insurance Code","Payor Id","Payor Subscriber Id","Eligibility Source","Room Coverage Type/Amount","Policy Type/Amount","Daily Deductible","Living Dependency","Ambulatory Status","Citizenship","Primary Language","Living Arrangement","Publicity Code","Protection Indicator","Student Indicator","Religion","Mother's Maiden Name","Nationality","Ethnic Group","Marital Status","Insured's Employment Start Date","Employment Stop Date","Job Title","Job Code/Class","Job Status","Employer Contact Person Name","Employer Contact Person Phone Number","Employer Contact Reason","Insured's Contact Person's Name","Insured's Contact Person Phone Number","Insured's Contact Person Reason","Relationship To The Patient Start Date","Relationship To The Patient Stop Date","Insurance Co Contact Reason","Insurance Co Contact Phone Number","Policy Scope","Policy Source","Patient Member Number","Guarantor's Relationship To Insured","Insured's Phone Number - Home","Insured's Employer Phone Number","Military Handicapped Program","Suspend Flag","Copay Limit Flag","Stoploss Limit Flag","Insured Organization Name And Id","Insured Employer Organization Name And Id","Race","Patient's Relationship To Insured"];
      let in3 = vec!["Set Id - In3","Certification Number","Certified By","Certification Required","Penalty","Certification Date/Time","Certification Modify Date/Time","Operator","Certification Begin Date","Certification End Date","Days","Non-concur Code/Description","Non-concur Effective Date/Time","Physician Reviewer","Certification Contact","Certification Contact Phone Number","Appeal Reason","Certification Agency","Certification Agency Phone Number","Pre-certification Requirement","Case Manager","Second Opinion Date","Second Opinion Status","Second Opinion Documentation Received","Second Opinion Physician"];
      let acc = vec!["Accident Date/Time","Accident Code","Accident Location","Auto Accident State","Accident Job Related Indicator","Accident Death Indicator","Entered By","Accident Description","Brought In By","Police Notified Indicator","Accident Address","Degree Of Patient Liability"];
      let ub1 = vec!["Set Id - Ub1","Blood Deductible","Blood Furnished-pints","Blood Replaced-pints","Blood Not Replaced-pints","Co-insurance Days","Condition Code","Covered Days","Non Covered Days","Value Amount & Code","Number Of Grace Days","Special Program Indicator","Psro/Ur Approval Indicator","Psro/Ur Approved Stay-fm","Psro/Ur Approved Stay-to","Occurrence","Occurrence Span","Occur Span Start Date","Occur Span End Date","Ub-82 Locator 2","Ub-82 Locator 9","Ub-82 Locator 27","Ub-82 Locator 45"];
      let ub2 = vec!["Set Id - Ub2","Co-insurance Days (9)","Condition Code (24-30)","Covered Days (7)","Non-covered Days (8)","Value Amount & Code","Occurrence Code & Date (32-35)","Occurrence Span Code/Dates (36)","UB92 Locator 2 (state)","UB92 Locator 11 (state)","UB92 Locator 31 (national)","Document Control Number","UB92 Locator 49 (national)","UB92 Locator 56 (state)","UB92 Locator 57 (national)","UB92 Locator 78 (state)","Special Visit Count"];
      let pda = vec!["Death Cause Code","Death Location","Death Certified Indicator","Death Certificate Signed Date/Time","Death Certified By","Autopsy Indicator","Autopsy Start And End Date/Time","Autopsy Performed By","Coroner Indicator"];
      let mrg = vec!["Prior Patient Identifier List", "Prior Alternate Patient Id","Prior Patient Account Number","Prior Patient Id","Prior Visit Number","Prior Alternate Visit Id","Prior Patient Name"];

      adt_database.insert(String::from("MSH"), msh);
      adt_database.insert(String::from("SFT"), sft);
      adt_database.insert(String::from("UAC"), uac);
      adt_database.insert(String::from("EVN"), evn);
      adt_database.insert(String::from("PID"), pid);
      adt_database.insert(String::from("PD1"), pd1);
      adt_database.insert(String::from("ARV"), arv);
      adt_database.insert(String::from("ROL"), rol);
      adt_database.insert(String::from("NK1"), nk1);
      adt_database.insert(String::from("PV1"), pv1);
      adt_database.insert(String::from("PV2"), pv2);
      adt_database.insert(String::from("DB1"), db1);
      adt_database.insert(String::from("OBX"), obx);
      adt_database.insert(String::from("AL1"), al1);
      adt_database.insert(String::from("DG1"), dg1);
      adt_database.insert(String::from("DRG"), drg);
      adt_database.insert(String::from("PR1"), pr1);
      adt_database.insert(String::from("GT1"), gt1);
      adt_database.insert(String::from("IN1"), in1);
      adt_database.insert(String::from("IN2"), in2);
      adt_database.insert(String::from("IN3"), in3);
      adt_database.insert(String::from("ACC"), acc);
      adt_database.insert(String::from("UB1"), ub1);
      adt_database.insert(String::from("UB2"), ub2);
      adt_database.insert(String::from("PDA"), pda);
      adt_database.insert(String::from("MRG"), mrg);



      let mut msgdetail = ParMsg::default(); 
      
      msgdetail.msgtype = msgtype;
 
 
     for (seg_i , seg) in msg.segments.iter().enumerate(){
         let mut msgvalue = MsgTypeField::default(); 
         let r ;
        for (fie_i, fie) in seg.fields.iter().enumerate(){
  
            // println!("{}",fie.components[0]); 
            //if Some(adt_database.get(&fie.components[0])) == fie.components[0].as_str() {
            if fie_i==0{
               msgvalue.segmentheader = fie.components[0].clone();
 
            }
        } 
        r = adt_database.get(&msgvalue.segmentheader).unwrap(); 
        if seg_i < 1{
           let mut count = 1;

           for (fie_i, fie) in seg.fields.iter().enumerate(){
               if fie_i >= 1{
                  if fie.components.len() == 1{
                     if fie.components[0] == "" {
                        count = count+1;
                        continue;
                     }else{

                        msgvalue.values.push((r[count].to_string(), fie.components[0].clone()));
                        count = count + 1;
                     }
                 }else{
                    let mut str = fie.components[0].clone();
                    //println!("{}",str);
                    for i in 1..fie.components.len(){
                        str.push_str("^");
                        str.push_str(&fie.components[i]);
                        //println!("{}",str);

                        }
                    msgvalue.values.push((r[count].to_string(), str));
                    count = count + 1;
                    }

                } 
            }
  
        }else{
            let mut count = 0;
           
            for (fie_i, fie) in seg.fields.iter().enumerate(){
                if fie_i >= 1{
                   if fie.components.len() == 1{
                      if fie.components[0] == "" {
                         count = count+1;
                         continue;
                       }else{

                         msgvalue.values.push((r[count].to_string(), fie.components[0].clone()));
                         count = count + 1;
                       }
                    }else{
                       let mut str = fie.components[0].clone();
                       //println!("{}",str);
                       for i in 1..fie.components.len(){
                           str.push_str("^");
                           str.push_str(&fie.components[i]);
                           //println!("{}",str);

                        }
                        msgvalue.values.push((r[count].to_string(), str));
                        count = count + 1;
                    }

                } 
            }
 
        }
        msgdetail.components.push(msgvalue); 
    }
 
    let jsoon = serde_json::to_string(&msgdetail).unwrap();
    jsoon

 }


 #[actix_rt::main]
async fn main() -> Result<(), io::Error>{

      // status();
      println!("http://127.0.0.1:8080");
      HttpServer::new(|| {
          App::new().route("/", web::get().to(status))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await

}
