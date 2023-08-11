
use std::io::Read;
use std::usize;
#[cfg(feature = "std")]
use std::vec::Vec;
#[cfg(not(feature = "std"))]
use core::cmp::Ordering;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use frost_secp256k1::DistributedKeyGeneration;
use frost_secp256k1::GroupKey;
use frost_secp256k1::IndividualPublicKey;
use frost_secp256k1::IndividualSecretKey;
use frost_secp256k1::SignatureAggregator;
use frost_secp256k1::compute_message_hash;
use frost_secp256k1::keygen;
use frost_secp256k1::keygen::SecretShare;
use frost_secp256k1::precomputation::CommitmentShare;
use frost_secp256k1::precomputation::PublicCommitmentShareList;
use frost_secp256k1::signature::Aggregator;
use frost_secp256k1::signature::Signer;
use k256::AffinePoint;
use k256::PublicKey;
use frost_secp256k1;
use frost_secp256k1::Participant;

use frost_secp256k1::Parameters;
use k256::Scalar;
use k256::Secp256k1;
use k256::SecretKey;
//use k256::elliptic_curve::PublicKey;
//use k256::elliptic_curve::PublicKey;
use k256::elliptic_curve::ScalarArithmetic;
use k256::elliptic_curve::group::GroupEncoding;
use rand::rngs::OsRng;
use rand::seq::index;
use sec1::point;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use serde;
use std::convert::TryInto;
use k256::ecdsa::Signature;
//use k256::ecdsa::signature::Signer;
use core::convert::TryFrom;
use generic_array::GenericArray;
use generic_array::typenum::Unsigned;
use crate::frost_secp256k1::generate_commitment_share_lists;
fn lines_from_file(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents
        .split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}

fn convert_secret_to_bytes(secretvector: &Vec<SecretShare>)->[u8;440]
{
    //Structure of Secretbytes 
    // every secret share is 44 bytes long 
    // loop through all bytes 
    let total=secretvector.len();
    let mut count=0;
    let mut secretbytes: [u8;440]=[0;440];
    let mut startindex=0;
    let mut endindex=0;
    while count<total
    {   
        let writebytes: Vec<u8>=bincode::serialize(&secretvector[count]).unwrap();
       // convert secret vector[count] to bytes
        let size: usize =writebytes.len();
        endindex=endindex+size;
        secretbytes[startindex..endindex].copy_from_slice(writebytes.as_slice());
         count=count+1;
        startindex=endindex;

    }
    secretbytes
}

fn convert_bytes_to_secret(secretbytes:[u8;440] )->Vec<SecretShare>
{
    //Structure of Secretbytes 
    // every secret share is 44 bytes long 
    // loop through all bytes 
    let mut secret_vector_from_bytes :Vec<SecretShare>=vec![];
    
     let mut startindex=0;
     let mut endindex=44;
     let mut total=11;
     let mut count=1;
     while count<total
    {   
        let mut bytesvalues: [u8;44]=[0;44];
        bytesvalues.copy_from_slice(&secretbytes[startindex..endindex]);
        let clone_secret_share: Result<SecretShare, Box<bincode::ErrorKind>>=bincode::deserialize(&bytesvalues);
        secret_vector_from_bytes.push(clone_secret_share.unwrap());
                count=count+1;
         startindex=endindex;
         endindex=endindex+44;

    }  
    secret_vector_from_bytes

}





//Line 110 for Main 
fn main() {

    let mut name = String::new();
    let mut threholdvalue :u32=7;
    let mut totalvalue :u32=3;
    let mut id: u32 = 1;
    
    println!("Kindly enter Current party value");
    let _=std::io::stdin().read_line(&mut name);
    // read params from file and assign them to id line0, thres line1 and totalvalue line2
    let lines = lines_from_file("src/params.txt");
    id =lines[0].trim().parse().unwrap();
    threholdvalue=lines[1].trim().parse().unwrap();
    totalvalue=lines[2].trim().parse().unwrap();
    //id=name.trim().parse().unwrap();  
    println!("id ={} , thresh={},total={}", id.to_string(), threholdvalue.to_string(), totalvalue.to_string());
  
    // create Directory for file 
    let mut pathfile = String::from("/opt/datafrost/") + lines[0].to_string().trim() + "/";
    let _res=fs::create_dir(&pathfile);
    let mut publickeytofile = pathfile + "public" + &lines[0].to_string() + ".txt";
    fs::remove_file(&publickeytofile).expect("could not remove file");
    let mut data_file = File::create(publickeytofile).expect("creation failed");

    // Create Participant using parameters
    let params = Parameters { n: totalvalue, t:threholdvalue };
    let   (mut party, _partycoeffs) = Participant::new(&params, id);
    //Convert Public key to bytes
        let public_bytes =party.public_key().unwrap().to_bytes();
        let _file_write_result=data_file.write_all(&public_bytes);
        let mut public_key_filepath = String::from("/opt/datafrost/")+ id.to_string().trim()  + "/public" + id.to_string().trim()+ ".txt";
        let mut file = match File::open(&public_key_filepath) {
            Ok(file) => file,
            Err(_) => panic!("no such file"),
        };
        //let mut bufferfile :[u8;65]=[0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4];
        let mut bufferfile: [u8;33]=[0;33];
          let _ = file.read_exact( &mut bufferfile);
        
          let testing3bytes=true;

    // Testing code with 3 Parties 
          if testing3bytes==true
    {
      // write code for 3 party testing
      
      let bytes_committed=convert_party3_to_bytes(&id, &party, &party.proof_of_secret_key);
      let partyconv=convert_bytes_to_party3(&bytes_committed);
      party.clone_from(&partyconv);
      // Clone to convert Z to zero in the original Party 

        
      let mut participantvectorpath = String::from("/opt/datafrost/") +&lines[0].to_string()+ "/participantvector" + &lines[0].to_string() + ".txt";
     
      println!("Verify the Participantvectorbinary file at {}",&participantvectorpath);
      fs::remove_file(&participantvectorpath).expect("could not remove file");
      let mut data_filecommit = File::create(&participantvectorpath).expect("creation failed"); // writing 
      let result_file_write=data_filecommit.write_all(&bytes_committed);
      
      
      let _=std::io::stdin().read_line(&mut name);
      let mut  other_Party_vectors: Vec<Participant>= vec!();
      let mut counter_party=1;
     // other_Party_vectors.clear();
      while (counter_party<4)
      {
          
          if counter_party==id
          {
              println!("Do nothing for self file creation");
          }
          else 
                          
          {
              let  path_to_read_party_vector = String::from("/opt/datafrost/") +&counter_party.to_string()+ "/participantvector" + &counter_party.to_string() + ".txt";
              let mut file = match File::open(&path_to_read_party_vector) {
                  Ok(file) => file,
                  Err(_) => panic!("no such file"),
              };
             // println!("{:?}",path_to_read_party_vector);
              let mut result_bytes_from_file:[u8;150]=[0;150];
              let result_read=file.read_exact(&mut result_bytes_from_file);

              //if result_read.is_ok()
              {
                  let mut party_input=convert_bytes_to_party3(&result_bytes_from_file);
                  
                  //println!("Value of Party vector {}",12-counter_party);

                  if party_input.index==party.index
                  {
                      println!("Dont push self key {} to Other party vector ",party_input.index)
                  }
                  else
                  {
                      println!("             ",);
                      println!("{:?}",party_input);
                       other_Party_vectors.push(party_input);
                  
                  
                  }
                
              }

              
          }
          counter_party=counter_party+1;

      }
      println!("{}",other_Party_vectors.len());
      println!("{}",counter_party);
      println!("waiting for DKG round 1");
      std::io::stdin().read_line(&mut name);


     // Go For DKG Part-1

      //DKG first Part  Round One 
      
      
      let mut partystate=DistributedKeyGeneration::<_>::new(&params,&id,&_partycoeffs,&mut other_Party_vectors).or(Err(())).unwrap();

      let mut partyone_secrets: &Vec<SecretShare>=partystate.their_secret_shares().unwrap();

      // println!("Secrets Vector Done for Id {} ",id);
      let total_secret_shares=partyone_secrets.len();
       println!("{:?}",partyone_secrets);
       println!("{:?}",partyone_secrets.len());
     
       let fullparty: [u8; 88]=convert_secret_to_3bytes(partyone_secrets);

       let mut secret_share_filepath = String::from("/opt/datafrost/")+ id.to_string().trim()  + "/party_secrets" + id.to_string().trim()+ ".txt";
       fs::remove_file(&secret_share_filepath).expect("could not remove file");
       let mut secret_file = File::create(&secret_share_filepath).expect("creation failed");
       let result=secret_file.write_all(&fullparty);

       println!("Checking all files are written with party scecrets");
       std::io::stdin().read_line(&mut name);
       

          // Start loop for retreiving secrets from all personnel
          // read all secret file vectors from other parties and select all secret shares with own id 
          let mut other_party_secret_shares: Vec<SecretShare>=vec!();
          let mut  file_nos=1;
          while file_nos<4
          {
          if file_nos==id
          {
              // no need to scan own file for own secret shares
              println!("no need to scan own file for own secret shares");
          }
          else {
          
          
           let mut secret_share_filepath = String::from("/opt/datafrost/")+ file_nos.to_string().trim()  + "/party_secrets" + file_nos.to_string().trim()+ ".txt";
           let mut file = match File::open(&secret_share_filepath) {
               Ok(file) => file,
               Err(_) => panic!("no such file"),
              };

          let mut secret_bytes : [u8;88]=[0;88];
          file.read_exact(&mut secret_bytes);
          let mut shared_vector=convert_bytes_to_3secret(secret_bytes);
          // find shares belonging to self from file 
          let mut vari_count=0;
          while (vari_count<shared_vector.len()+1)
          {
              if shared_vector[vari_count].index==id
              {println!("going through this file for self vector {}",secret_share_filepath);
                  other_party_secret_shares.push(shared_vector[vari_count].clone());
                  
                  break;
              }
              vari_count=vari_count+1;

          }


              } // else 
              file_nos=file_nos+1;
              
          } // while reading all files

         
          let  partystate2: DistributedKeyGeneration<keygen::RoundOne>=partystate.clone();
      
          let  partystaternd2: Result<DistributedKeyGeneration<keygen::RoundTwo>, ()> = partystate2.clone().to_round_two( other_party_secret_shares);
          
          let partystaternd2: DistributedKeyGeneration<keygen::RoundTwo>=partystaternd2.unwrap();
  
          let mut partyfinale=partystaternd2.finish(&party.public_key().unwrap()).unwrap();
          
        //   //let mut partysecretkey=blabblabbalb.as_mut().unwrap().1;
        //   println!("Groupkey");
        //   println!("{:?}",Partyfinale.0);
           
  
        //    println!("Secret key full ");
        //     println!("{:?}",&mut Partyfinale.1);
        //   // println!("{:?}",&mut blabblabbalb.1);
        //    println!("Public key from Private key ");
        //    println!("{:?}",&mut Partyfinale.1.to_public());

           println!("Groupkey");
           println!("{:?}",partyfinale.0);
           
   
            println!("Secret key full ");
             println!("{:?}",&mut partyfinale.1);
           // println!("{:?}",&mut blabblabbalb.1);
            println!("Public key from Private key ");
            println!("{:?}",&mut partyfinale.1.to_public());
            
            println!("Groupkey bytes");
            println!("{:?}",partyfinale.0.to_bytes()); 
            println!("Secret key bytes ");
            println!("{:?}",partyfinale.1.key.to_bytes());
            println!("Public key bytes ");
            println!("{:?}",partyfinale.1.to_public().share.to_bytes());
            let custom=partyfinale.1.to_public();

            // Move to Partial Signature Creation 
            // Assign Party 1 as leader 
            //
            let context = b"CONTEXT STRING FOR XP NFT BRIDGE TEST FOR APPLE>D>HAIDER>SMITH";
            let message = b"This is a test message from Xp Bridge piece Meal 20230815";

            if id==1
            {
                // do some special work if id ==1 
                let (mut p1_public_comshares, mut p1_secret_comshares) = generate_commitment_share_lists(&mut OsRng, id, 1);
               // let signerres=aggregator.get_signers();
               println!("Inside agregator loop ");       
                let mut aggregator=SignatureAggregator::new(params,partyfinale.0,context.to_vec(),message.to_vec());
                
                //println!("{:?}",p1_public_comshares.commitments[0].0.to_bytes());
                //println!("{:?}",p1_public_comshares.commitments[0].0.to_bytes().len());
                //let sample=bincode::serialize(&p1_public_comshares.commitments[0]);
                //let pubkey: Result<PublicCommitmentShareList, Box<bincode::ErrorKind>>=bincode::deserialize(&sample.unwrap());
                //let value=AffinePoint::from_bytes(&p1_public_comshares.commitments[0].0.to_bytes());
                //println!("{:?}",value.unwrap().to_bytes());
                //let value=AffinePoint::from_bytes(&p1_public_comshares.commitments[0].1.to_bytes());
                //println!("{:?}",value.unwrap().to_bytes());
                println!("Bytes commitments ");      
                let bytesoff: [u8; 70] =public_commitment_to_bytes(&p1_public_comshares);

                
            //    println!("{:?}",bytesoff);
            //    println!("***********************");
            //    println!("***********************");
            //    println!("***********************");
            //    println!("{:?}",p1_public_comshares);
            //    println!("***********************");
            //    println!("*****From return function*****");
            //    println!("{:?}",public_bytes_to_commitment(bytesoff));
               //let xya:PublicCommitmentShareList=new PublicCommitmentShareList();
            //    xya.participant_index=1;
            //    xya.commitments[0].0.clone_from(&value.unwrap());

               // let xy=PublicCommitmentShareList::from(p1_public_comshares);
                //println!("{:?}",pubkey.unwrap().commitments[0].0.to_bytes());

                //let xyz:PublicCommitmentShareList;
               // xyz.participant_index=1;
                //let value=AffinePoint::to_bytes(&p1_public_comshares.commitments[0].0);

                //let part1=p1_public_comshares.commitments[0].0.to_bytes();
                //let part2=p1_public_comshares.commitments[0].1.to_bytes();

                // write commitment share and public key in files
                let mut public_comshare_filepath = String::from("/opt/datafrost/")+ id.to_string().trim()  + "/public_comshares" + id.to_string().trim()+ ".txt";
              //  fs::remove_file(&public_comshare_filepath).expect("could not remove file");
                println!("{}",public_comshare_filepath);
                let mut secret_file = File::create(&public_comshare_filepath).expect("creation failed");
                let result=secret_file.write_all(&bytesoff);
                
                let mut public_keyshare_filepath = String::from("/opt/datafrost/")+ id.to_string().trim()  + "/public_final_key" + id.to_string().trim()+ ".txt";
              //  fs::remove_file(&public_comshare_filepath).expect("could not remove file");
                println!("{}",public_keyshare_filepath);
                let mut secret_file = File::create(&public_keyshare_filepath).expect("creation failed");
                
                let result=secret_file.write_all(&partyfinale.1.to_public().share.to_bytes());
                //PublicKey::from_sec1_bytes(bytes)
                //partyfinale.1.to_public().share.to_bytes()
                let mut partynew=partyfinale;
                
                //AffinePoint::from_bytes()
                //let alpha :IndividualPublicKey=IndividualPublicKey { index: (), share: }
                //partynew.1.to_public().clone_from(source)
                let message_hash = compute_message_hash(&context[..], &message[..]);
              //  let signers = aggregator.get_signers();
                
                // loop through all other files for commitment and public key 
                let mut count=2;
                while count <4
                {
                    let mut public_comshare_filepath = String::from("/opt/datafrost/")+ count.to_string().trim()  + "/public_comshares" + count.to_string().trim()+ ".txt";
                    let mut bytespublicexact: [u8; 70]=[0;70];
                    let mut file_pub = match File::open(&public_comshare_filepath) {
                        Ok(file_pub) => file_pub,
                        Err(_) => panic!("no such file"),
                       };
                     file_pub.read_exact(&mut bytespublicexact);
                               
                    let comms=public_bytes_to_commitment(bytespublicexact);
                    //let commbnew:PublicCommitmentShareList=PublicCommitmentShareList { participant_index: comms.participant_index, commitments: comms.commitments };

                    // get commitment share list from bytes

                    let mut public_keyshare_filepath = String::from("/opt/datafrost/")+ count.to_string().trim()  + "/public_final_key" + count.to_string().trim()+ ".txt";
                    let mut bytespublickey: [u8; 33]=[0;33];
                    let mut file_pubkey = match File::open(&public_keyshare_filepath) {
                        Ok(file_pubkey) => file_pubkey,
                        Err(_) => panic!("no such file"),
                       };
                     file_pubkey.read_exact(&mut bytespublickey);

                       println!("{:?}",bytespublickey);
                     let mut genarraypublic=GenericArray::from_slice(&bytespublickey);
                     println!("{:?}",genarraypublic);

                     let pk_sk_affinepoint=AffinePoint::from_bytes(genarraypublic);
                     let pk_sk_affinepoint=pk_sk_affinepoint.unwrap();
                     partynew.1.to_public().share.clone_from(&pk_sk_affinepoint);

                      let publickey_party_count=PublicKey::from_affine(pk_sk_affinepoint).unwrap();
                    
                    let alpha : IndividualPublicKey=IndividualPublicKey { index: count, share: pk_sk_affinepoint };
                    let xyz=PublicKey::from_affine(pk_sk_affinepoint);
                    let xyz= xyz.unwrap();

                aggregator.include_signer(count, comms.commitments[0],alpha);     
                
                   
                    //let publickey_party_count=IndividualPublicKey::clone_from(&mut self, source)
                     
                     count=count+1;

                }


                //aggregator.include_signer(1, p1_public_comshares.commitments[0], (&p1_sk).into());
                //aggregator.include_signer(3, p3_public_comshares.commitments[0], (&p3_sk).into());
                //aggregator.include_signer(4, p4_public_comshares.commitments[0], (&p4_sk).into());
                  let signers: &Vec<frost_secp256k1::signature::Signer> = aggregator.get_signers();
                  
                //println!("{:?}",signers);
                println!("{:?}",signers);
                let bytessamoke=signer_vector_tobytes(signers, 0);
                println!("{:?}",bytessamoke);
                println!("{:?}",signer_bytes_tovector(bytessamoke));
                let mut signer_140_file = String::from("/opt/datafrost/")+ "signer_vector_140" + ".txt";
                //  fs::remove_file(&public_comshare_filepath).expect("could not remove file");
                  //println!("{}",public_keyshare_filepath);
                  let mut signer_file_writer = File::create(&signer_140_file).expect("creation failed");
                    signer_file_writer.write_all(&bytessamoke);
                    println!("signer bytes written ");
                    println!( "go ahead on signers for writing Partial signatures ");
                             std::io::stdin().read_line(&mut name);
                
                

                
                //partyfinale.1.sign(&message_hash, group_key, my_secret_commitment_share_list, my_commitment_share_index, signers)
            }
            else {
                let (mut p1_public_comshares, mut p1_secret_comshares) = generate_commitment_share_lists(&mut OsRng, id, 1);
                //let (any_public_comshares, mut any_secret_comshares) = generate_commitment_share_lists(&mut OsRng, id, 1);
                // write commitment share and public key in files

                let message_hash = compute_message_hash(&context[..], &message[..]);
             //   let party_partial = partyfinale.1.sign(&message_hash, &partyfinale.0,&mut p1_secret_comshares,0,&signers).unwrap();
                 let mut public_comshare_filepath = String::from("/opt/datafrost/")+ id.to_string().trim()  + "/public_comshares" + id.to_string().trim()+ ".txt";
             //  fs::remove_file(&public_comshare_filepath).expect("could not remove file");
               println!("{}",public_comshare_filepath);
               let mut secret_file = File::create(&public_comshare_filepath).expect("creation failed");
               let bytesoff: [u8; 70] =public_commitment_to_bytes(&p1_public_comshares);
               let result=secret_file.write_all(&bytesoff);
               
               let mut public_keyshare_filepath = String::from("/opt/datafrost/")+ id.to_string().trim()  + "/public_final_key" + id.to_string().trim()+ ".txt";
             //  fs::remove_file(&public_comshare_filepath).expect("could not remove file");
               println!("{}",public_keyshare_filepath);
               let mut secret_file = File::create(&public_keyshare_filepath).expect("creation failed");
               
               let result=secret_file.write_all(&partyfinale.1.to_public().share.to_bytes());
               
               println!( "go ahead on signers for writing Partial signatures ");
                        std::io::stdin().read_line(&mut name);

               //read signers vector from file 
               let mut signer_140_file = String::from("/opt/datafrost/")+ "signer_vector_140" + ".txt";
               let mut signer_140_bytes: [u8; 140]=[0;140];
                    let mut file_signer = match File::open(&signer_140_file) {
                        Ok(file_signer) => file_signer,
                        Err(_) => panic!("no such file"),
                       };
                       file_signer.read_exact(&mut signer_140_bytes);

                       let signer_140_from_file=signer_bytes_tovector(signer_140_bytes);
               let party_partial = partyfinale.1.sign(&message_hash, &partyfinale.0,&mut p1_secret_comshares,0,&signer_140_from_file).unwrap();
                       println!("{:?}", party_partial);
                
            }
             /* 
            
            
                
                
                aggregator.include_signer(1,p1_public_comshares.commitments[0],partyfinale.1.to_public());
            
                let messagehash=compute_message_hash(context, message);
    
    
    */  
        /*
    
            aggregator.include_signer(1, p1_public_comshares.commitments[0], (&p1_sk).into());
            aggregator.include_signer(3, p3_public_comshares.commitments[0], (&p3_sk).into());
            aggregator.include_signer(4, p4_public_comshares.commitments[0], (&p4_sk).into());
    
            
            let message_hash = compute_message_hash(&context[..], &message[..]);
            let signers = aggregator.get_signers();
            let p1_partial = p1_sk.sign(&message_hash, &group_key, &mut p1_secret_comshares, 0, signers).unwrap();
            let p3_partial = p3_sk.sign(&message_hash, &group_key, &mut p3_secret_comshares, 0, signers).unwrap();
            let p4_partial = p4_sk.sign(&message_hash, &group_key, &mut p4_secret_comshares, 0, signers).unwrap();
    
            aggregator.include_partial_signature(p1_partial);
            aggregator.include_partial_signature(p3_partial);
            aggregator.include_partial_signature(p4_partial);
    
            let aggregator = aggregator.finalize().unwrap();
            let threshold_signature = aggregator.aggregate().unwrap();
            let verification_result = threshold_signature.verify(&group_key, &message_hash);
    
            assert!(verification_result.is_ok())
        
        
         
    
         ! # let (alice_public_comshares, mut alice_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 1, 1);
    //! # let (bob_public_comshares, mut bob_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 2, 1);
    //! # let (carol_public_comshares, mut carol_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 3, 1);
    //! #
    //! # let context = b"CONTEXT STRING STOLEN FROM DALEK TEST SUITE";
    //! # let message = b"This is a test of the tsunami alert system. This is only a test.";
    //! #
    //! # let message_hash = compute_message_hash(&context[..], &message[..]);
    //! #
    //! # let mut aggregator = SignatureAggregator::new(params, bob_group_key.clone(), context.to_vec(), message.to_vec());
    //! #
    //! # aggregator.include_signer(1, alice_public_comshares.commitments[0], (&alice_secret_key).into());
    //! # aggregator.include_signer(3, carol_public_comshares.commitments[0], (&carol_secret_key).into());
    //! #
    //! # let signers = aggregator.get_signers();
    //!
    //! let alice_partial = alice_secret_key.sign(&message_hash, &alice_group_key,
    //!                                           &mut alice_secret_comshares, 0, signers)?;
    //! let carol_partial = carol_secret_key.sign(&message_hash, &carol_group_key,
    //!                                           &mut carol_secret_comshares, 0, signers)?;
    //!
    //! aggregator.include_partial_signature(alice_partial);
    //! aggregator.include_partial_signature(carol_partial);
    //! # Ok(()) }
    //! # #[cfg(feature = "std")]
    //! # fn main() { assert!(do_test().is_ok()); }
    //! # #[cfg(not(feature = "std"))]
    //! # fn main() { }
    //! ```
    //!
    //! ## Signature Aggregation
    //!
    //! Once all the expected signers have sent their partial signatures, the
    //! aggregator attempts to finalize its state, ensuring that there are no errors
    //! thus far in the partial signatures, before finally attempting to complete
    //! the aggregation of the partial signatures into a threshold signature.
    //!
    //! ```rust,ignore
    //! let aggregator = aggregator.finalize()?;
    //! ```
    //!
    //! If the aggregator could not finalize the state, then the `.finalize()` method
    //! will return a `HashMap<u32, &'static str>` describing participant indices and the issues
    //! encountered for them.  These issues are **guaranteed to be the fault of the aggregator**,
    //! e.g. not collecting all the expected partial signatures, accepting two partial
    //! signatures from the same participant, etc.
    //!
    //! And the same for the actual aggregation, if there was an error then a
    //! `HashMap<u32, &'static str>` will be returned which maps participant indices to issues.
    //! Unlike before, however, these issues are guaranteed to be the fault of the
    //! corresponding participant, specifically, that their partial signature was invalid.
    //!
    //! ```rust,ignore
    //! let threshold_signature = aggregator.aggregate()?;
    //! ```
    //!
    //! Anyone with the group public key can then verify the threshold signature
    //! in the same way they would for a standard Schnorr signature.
    //!
    //! ```rust,ignore
    //! let verified = threshold_signature.verify(&alice_group_key, &message_hash)?;
    
    
    */



              
    }
          else {
            // full party 440 
            /*
            Full mode 11       
            ________________
            /.,------------,.\
            ///  .=^^^^^^^\__|\\
            \\\   `------.   .//
            `\\`--...._  `;//'
            `\\.-,___;.//'
                `\\-..-//'
            ZKP    `\\//'
                    ""

            */
        let bytes_committed=convert_party_to_bytes(&id, &party, &party.proof_of_secret_key);
      
         let mut participantvectorpath = String::from("/opt/datafrost/") +&lines[0].to_string()+ "/participantvector" + &lines[0].to_string() + ".txt";
            println!("Verify the Participantvectorbinary file at {}",&participantvectorpath);
    //         std::io::stdin().read_line(&mut name);
            fs::remove_file(&participantvectorpath).expect("could not remove file");
         let mut data_filecommit = File::create(&participantvectorpath).expect("creation failed"); // writing 
         let result_file_write=data_filecommit.write_all(&bytes_committed);
                // Convert original party to 32 bytes party with z value zero 
         let partyconv=convert_bytes_to_party(&bytes_committed);
         party.clone_from(&partyconv);
    
    // Create all files for computation if filler = ture
        let mut filler =false;
        let mut file_nos=1;
       while file_nos<12 && filler== true
       {
           if file_nos==id
           {

           }
           else {
               
           
           let mut pathfile = String::from("/opt/datafrost/") + &file_nos.to_string().trim() + "/";
           let _res=fs::create_dir(&pathfile);
           let mut publickeytofile = pathfile + "public" + &file_nos.to_string() + ".txt";
           let mut data_file = File::create(publickeytofile).expect("creation failed");
       
           // Create Participant using parameters
           let params = Parameters { n: totalvalue, t:threholdvalue };
           let (party_to_write, _partycoeffs) = Participant::new(&params, file_nos);
           //Convert Public key to bytes
               let public_bytes =party_to_write.public_key().unwrap().to_bytes();
               let _file_write_result=data_file.write_all(&public_bytes);
               let mut public_key_filepath = String::from("/opt/datafrost/")+ file_nos.to_string().trim()  + "/public" + file_nos.to_string().trim()+ ".txt";
               let mut file = match File::open(&public_key_filepath) {
                   Ok(file) => file,
                   Err(_) => panic!("no such file"),
               };
               let bytes_committed=convert_party_to_bytes(&file_nos, &party_to_write, &party_to_write.proof_of_secret_key);

                    
                let mut participantvectorpath = String::from("/opt/datafrost/") +&file_nos.to_string()+ "/participantvector" + &file_nos.to_string() + ".txt";
               
                println!("Verify the Participantvectorbinary file at {}",&participantvectorpath);
                //std::io::stdin().read_line(&mut name);
             
                let mut data_filecommit = File::create(&participantvectorpath).expect("creation failed"); // writing 
                let result_file_write=data_filecommit.write_all(&bytes_committed);

           }
           file_nos=file_nos+1;
       }// Files Creation Loop ends


        //let partyglobal=convert_bytes_to_party(&bytes_committed);
                 std::io::stdin().read_line(&mut name);
        let mut  other_Party_vectors: Vec<Participant>= vec!();
        let mut counter_party=1;
       // other_Party_vectors.clear();
       // Get shares from all party vectors 
        while (counter_party<12)
        {
          
            if counter_party==id
            {
                println!("Do nothing for self file creation");
            }
            else 
                            
            {
                let  path_to_read_party_vector = String::from("/opt/datafrost/") +&counter_party.to_string()+ "/participantvector" + &counter_party.to_string() + ".txt";
                let mut file = match File::open(&path_to_read_party_vector) {
                    Ok(file) => file,
                    Err(_) => panic!("no such file"),
                };
               // println!("{:?}",path_to_read_party_vector);
                let mut result_bytes_from_file:[u8;315]=[0;315];
                let result_read=file.read_exact(&mut result_bytes_from_file);

                //if result_read.is_ok()
                {
                    let mut party_input=convert_bytes_to_party(&result_bytes_from_file);
                    
                    //println!("Value of Party vector {}",12-counter_party);

                    if party_input.index==party.index
                    {
                        println!("Dont push self key {} to Other party vector ",party_input.index)
                    }
                    else
                    {
                        //println!("             ",);
                        //println!("{:?}",party_input);
                         other_Party_vectors.push(party_input);
                    
                    
                    }
                  
                }

                
            }
            counter_party=counter_party+1;

        }
        //println!("{}",other_Party_vectors.len());
        //println!("{}",counter_party);
        std::io::stdin().read_line(&mut name);

       // Go For DKG Part-1

        //DKG first Part  Round One 
        // with mulit parties
        
        
        let mut partystate=DistributedKeyGeneration::<_>::new(&params,&id,&_partycoeffs,&mut other_Party_vectors).or(Err(())).unwrap();

        let mut partyone_secrets: &Vec<SecretShare>=partystate.their_secret_shares().unwrap();

        // println!("Secrets Vector Done for Id {} ",id);
        let total_secret_shares=partyone_secrets.len();
       //  println!("{:?}",partyone_secrets);
      //   println!("{:?}",partyone_secrets.len());
      // full party 440 
            /*
            Full mode 11       
            ________________
            /.,------------,.\
            ///  .=^^^^^^^\__|\\
            \\\   `------.   .//
            `\\`--...._  `;//'
            `\\.-,___;.//'
                `\\-..-//'
            ZKP    `\\//'
                    ""




            */
                    
        //Write own Share to file 
        
        let fullparty=convert_secret_to_bytes(partyone_secrets);

         let mut secret_share_filepath = String::from("/opt/datafrost/")+ id.to_string().trim()  + "/party_secrets" + id.to_string().trim()+ ".txt";
         fs::remove_file(&secret_share_filepath).expect("could not remove file");
         let mut secret_file = File::create(&secret_share_filepath).expect("creation failed");
         let result=secret_file.write_all(&fullparty);
         println!("Checking all files are written with party scecrets");
         std::io::stdin().read_line(&mut name);
         

            // Start loop for retreiving secrets from all personnel
            // read all secret file vectors from other parties and select all secret shares with own id 
            let mut other_party_secret_shares: Vec<SecretShare>=vec!();
            let mut  file_nos=1;
            while file_nos<12
            {
            if file_nos==id
            {
                // no need to scan own file for own secret shares
                println!("no need to scan own file for own secret shares");
            }
            else {
            
            
             let mut secret_share_filepath = String::from("/opt/datafrost/")+ file_nos.to_string().trim()  + "/party_secrets" + file_nos.to_string().trim()+ ".txt";
             let mut file = match File::open(&secret_share_filepath) {
                 Ok(file) => file,
                 Err(_) => panic!("no such file"),
                };

            let mut secret_bytes : [u8;440]=[0;440];
            file.read_exact(&mut secret_bytes);
            let mut shared_vector=convert_bytes_to_secret(secret_bytes);
            // find shares belonging to self from file 
            let mut vari_count=0;
            while (vari_count<shared_vector.len()+1)
            {//println!("going through this file {}",secret_share_filepath);
                if shared_vector[vari_count].index==id
                {
                    other_party_secret_shares.push(shared_vector[vari_count].clone());
                    
                    break; // only one entry of self in any shared secret vector file
                }
                vari_count=vari_count+1;

            }


                } // else 
                file_nos=file_nos+1;
                
            } // while reading all files

           
            let  partystate2: DistributedKeyGeneration<keygen::RoundOne>=partystate.clone();
        
            let  partystaternd2: Result<DistributedKeyGeneration<keygen::RoundTwo>, ()> = partystate2.clone().to_round_two( other_party_secret_shares);
            
            let partystaternd2: DistributedKeyGeneration<keygen::RoundTwo>=partystaternd2.unwrap();
    
            let mut partyfinale=partystaternd2.finish(&party.public_key().unwrap()).unwrap();
            
            //let mut partysecretkey=blabblabbalb.as_mut().unwrap().1;
            println!("Groupkey");
            println!("{:?}",partyfinale.0);
            
    
             println!("Secret key full ");
              println!("{:?}",&mut partyfinale.1);
            // println!("{:?}",&mut blabblabbalb.1);
             println!("Public key from Private key ");
             println!("{:?}",&mut partyfinale.1.to_public());
             
             println!("Groupkey bytes");
             println!("{:?}",partyfinale.0.to_bytes()); 
             println!("Secret key bytes ");
             println!("{:?}",partyfinale.1.key.to_bytes());
             println!("Public key bytes ");
             println!("{:?}",partyfinale.1.to_public().share.to_bytes());
             //println!("{:?}", partyfinale

             //println!("{:?}",&mut Partyfinale
           // println!("{:?}",&mut blabblabbalb.1);
            // println!("Public key from Private key ");
            // println!("{:?}",&mut Partyfinale.1.to_public());
    
    // need for signing 
/*
            ,....,
            ,::::::<
        ,::/^\"``.
        ,::/, `   e`.
        ,::; |        '.
        ,::|  \___,-.  c)
        ;::|     \   '-'
            
 */
    /* 
        let context = b"CONTEXT STRING FOR XP NFT BRIDGE TEST FOR APPLE>D>HAIDER>SMITH";

        let message = b"This is a test message from Xp Bridge piece Meal 20230815";
        let context = b"CONTEXT STRING STOLEN FROM DALEK TEST SUITE";
        let message = b"This is a test of the tsunami alert system. This is only a test.";
        let (p1_public_comshares, mut p1_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 1, 1);
        let (p3_public_comshares, mut p3_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 3, 1);
        let (p4_public_comshares, mut p4_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 4, 1);
            
            let mut aggregator=SignatureAggregator::new(params,partyfinale.0,context.to_vec(),message.to_vec());
            aggregator.include_signer(1,p1_public_comshares.commitments[0],partyfinale.1.to_public());
            let signerres=aggregator.get_signers();
            let messagehash=compute_message_hash(context, message);


*/  
    /*
       let (group_key, p1_sk) = p1_state.finish(&p1.public_key().unwrap()).unwrap();
        let (_, _) = p2_state.finish(&p2.public_key().unwrap()).unwrap();
        let (_, p3_sk) = p3_state.finish(&p3.public_key().unwrap()).unwrap();
        let (_, p4_sk) = p4_state.finish(&p4.public_key().unwrap()).unwrap();
        let (_, _) = p5_state.finish(&p5.public_key().unwrap()).unwrap();

        let context = b"CONTEXT STRING STOLEN FROM DALEK TEST SUITE";
        let message = b"This is a test of the tsunami alert system. This is only a test.";
        let (p1_public_comshares, mut p1_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 1, 1);
        let (p3_public_comshares, mut p3_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 3, 1);
        let (p4_public_comshares, mut p4_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 4, 1);

        let mut aggregator = SignatureAggregator::new(params, group_key, context.to_vec(), message.to_vec());

        aggregator.include_signer(1, p1_public_comshares.commitments[0], (&p1_sk).into());
        aggregator.include_signer(3, p3_public_comshares.commitments[0], (&p3_sk).into());
        aggregator.include_signer(4, p4_public_comshares.commitments[0], (&p4_sk).into());

        let signers = aggregator.get_signers();
        let message_hash = compute_message_hash(&context[..], &message[..]);

        let p1_partial = p1_sk.sign(&message_hash, &group_key, &mut p1_secret_comshares, 0, signers).unwrap();
        let p3_partial = p3_sk.sign(&message_hash, &group_key, &mut p3_secret_comshares, 0, signers).unwrap();
        let p4_partial = p4_sk.sign(&message_hash, &group_key, &mut p4_secret_comshares, 0, signers).unwrap();

        aggregator.include_partial_signature(p1_partial);
        aggregator.include_partial_signature(p3_partial);
        aggregator.include_partial_signature(p4_partial);

        let aggregator = aggregator.finalize().unwrap();
        let threshold_signature = aggregator.aggregate().unwrap();
        let verification_result = threshold_signature.verify(&group_key, &message_hash);

        assert!(verification_result.is_ok())
    
    
     

     ! # let (alice_public_comshares, mut alice_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 1, 1);
//! # let (bob_public_comshares, mut bob_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 2, 1);
//! # let (carol_public_comshares, mut carol_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 3, 1);
//! #
//! # let context = b"CONTEXT STRING STOLEN FROM DALEK TEST SUITE";
//! # let message = b"This is a test of the tsunami alert system. This is only a test.";
//! #
//! # let message_hash = compute_message_hash(&context[..], &message[..]);
//! #
//! # let mut aggregator = SignatureAggregator::new(params, bob_group_key.clone(), context.to_vec(), message.to_vec());
//! #
//! # aggregator.include_signer(1, alice_public_comshares.commitments[0], (&alice_secret_key).into());
//! # aggregator.include_signer(3, carol_public_comshares.commitments[0], (&carol_secret_key).into());
//! #
//! # let signers = aggregator.get_signers();
//!
//! let alice_partial = alice_secret_key.sign(&message_hash, &alice_group_key,
//!                                           &mut alice_secret_comshares, 0, signers)?;
//! let carol_partial = carol_secret_key.sign(&message_hash, &carol_group_key,
//!                                           &mut carol_secret_comshares, 0, signers)?;
//!
//! aggregator.include_partial_signature(alice_partial);
//! aggregator.include_partial_signature(carol_partial);
//! # Ok(()) }
//! # #[cfg(feature = "std")]
//! # fn main() { assert!(do_test().is_ok()); }
//! # #[cfg(not(feature = "std"))]
//! # fn main() { }
//! ```
//!
//! ## Signature Aggregation
//!
//! Once all the expected signers have sent their partial signatures, the
//! aggregator attempts to finalize its state, ensuring that there are no errors
//! thus far in the partial signatures, before finally attempting to complete
//! the aggregation of the partial signatures into a threshold signature.
//!
//! ```rust,ignore
//! let aggregator = aggregator.finalize()?;
//! ```
//!
//! If the aggregator could not finalize the state, then the `.finalize()` method
//! will return a `HashMap<u32, &'static str>` describing participant indices and the issues
//! encountered for them.  These issues are **guaranteed to be the fault of the aggregator**,
//! e.g. not collecting all the expected partial signatures, accepting two partial
//! signatures from the same participant, etc.
//!
//! And the same for the actual aggregation, if there was an error then a
//! `HashMap<u32, &'static str>` will be returned which maps participant indices to issues.
//! Unlike before, however, these issues are guaranteed to be the fault of the
//! corresponding participant, specifically, that their partial signature was invalid.
//!
//! ```rust,ignore
//! let threshold_signature = aggregator.aggregate()?;
//! ```
//!
//! Anyone with the group public key can then verify the threshold signature
//! in the same way they would for a standard Schnorr signature.
//!
//! ```rust,ignore
//! let verified = threshold_signature.verify(&alice_group_key, &message_hash)?;


*/

































}



fn convert_party_to_bytes(index: &u32, commitments_party: &frost_secp256k1::Participant,zkp:&frost_secp256k1::nizk::NizkOfSecretKey) -> [u8;315]{

        // Structure of bytes
        // ZKP R scaler 40 bytes
        // ZKP S scaler 40 bytes
        // 7 Commitments shares 33 bytes=231
        // index u32 ->u8 = 4 bytes
        // Total=40+40+33+33+33+33+33+33+33+4=315 

    let mut resultbytes:[u8;315]=[0;315];
    //let mut resultdummy: [u8;40]=[0;40];
    let rbytes=bincode::serialize(&zkp.r).unwrap();
    let split=rbytes.split_at(40);
    resultbytes[0..40].clone_from_slice(&split.0);
    //copy R bytes to resulant bytes
    let sbytes=bincode::serialize(&zkp.s).unwrap();
    let split=sbytes.split_at(40);
    resultbytes[40..80].clone_from_slice(&split.0);
    //copy S bytes to resulant bytes
    
    let mut commit_count=0;
    let mut startin_byte_index=80;
    // start loop to copy all commitment vectors to resulant bytes
    while commit_count<7
    {   let ending_index=startin_byte_index+33;
        let commitmentbytes=commitments_party.commitments[commit_count].to_bytes();
        let commit_split=commitmentbytes.split_at(33);
        resultbytes[startin_byte_index..ending_index].clone_from_slice(commit_split.0);
        startin_byte_index=ending_index;
        commit_count=commit_count+1;

    }
    // copy index bytes in the resultant buffer
    resultbytes[startin_byte_index..315].copy_from_slice(index.to_be_bytes().as_slice());
    //return resultbytes
    resultbytes
}
pub struct ZKPSecretKey {
    /// The scalar portion of the Schnorr signature encoding the context.
    pub s: Scalar,
    /// The scalar portion of the Schnorr signature which is the actual signature.
    pub r: Scalar,
}


fn convert_bytes_to_party(party_bytes: &[u8;315]) -> (Participant)
{
    // Structure of bytes
        // ZKP R scaler 40 bytes
        // ZKP S scaler 40 bytes
        // 7 Commitments shares 33 bytes=231
        // index u32 ->u8 = 4 bytes
        // Total=40+40+33+33+33+33+33+33+33+4=315 
    let mut commit_vector:Vec<k256::ProjectivePoint>=vec!();
    
    let mut bytes_sequence :[u8;4]=[0,0,0,0];
    bytes_sequence.clone_from_slice(&party_bytes[311..315]);

    
    //let value_index=indexconvert as u32
    // convert u8 to u32 formation of Index
    let index_u32_integer: u32 = ((bytes_sequence[0] as u32) << 24)
                    | ((bytes_sequence[1] as u32) << 16)
                    | ((bytes_sequence[2] as u32) << 8)
                    | (bytes_sequence[3] as u32);;
   
    let mut bytes_for_r: [u8;40]=[0;40];
    let mut bytes_for_s:[u8;40]=[0;40];
    // copy r and s bytes from slice 
    bytes_for_r.copy_from_slice(&party_bytes[0..40]);
    bytes_for_s.copy_from_slice(&party_bytes[40..80]);

    // create S and R from De-Serializer 
          
    let  skey: Result<Scalar, Box<bincode::ErrorKind>>  =bincode::deserialize(bytes_for_s.as_ref());
    let  rkey: Result<Scalar, Box<bincode::ErrorKind>>  =bincode::deserialize(bytes_for_r.as_ref());
    // create a new zkp with r and S for formation of participant
    let mut zkpfull :frost_secp256k1::nizk::NizkOfSecretKey= frost_secp256k1::nizk::NizkOfSecretKey { s: skey.unwrap(), r: rkey.unwrap() };
    
    let mut commit=0;
    let mut start_bytes=80;
    // loop through commitment of  33 bytes to get all commitment vectors
    while(commit<7)
    {
        let endvalue=start_bytes+33;
        let mut bytescommit:[u8;33]=[0;33];
        
        
       
        bytescommit.copy_from_slice(&party_bytes[start_bytes..endvalue]);
        let mut genarray=GenericArray::from_slice(bytescommit.as_ref());
       
        let mut byte_projective=k256::ProjectivePoint::from_bytes(&genarray).unwrap(); 
               
        commit_vector.push(byte_projective);

        start_bytes=endvalue;
        commit=commit+1;

    }
    
        let mut party_convert: Participant=Participant { index: index_u32_integer , commitments: commit_vector, proof_of_secret_key: zkpfull };


party_convert
}


    fn convert_party_to_bytes2(index: &u32, commitments_party: &frost_secp256k1::Participant,zkp:&frost_secp256k1::nizk::NizkOfSecretKey) -> [u8;299]{



        let mut resultbytes:[u8;299]=[0;299];
        // Structure of bytes
        // ZKP R scaler 32 bytes
        // ZKP S scaler 32 bytes
        // 7 Commitments shares 33 bytes=231
        // index u32 ->u8 = 4 bytes
        // Total=32+32+33+33+33+33+33+33+33+4=299 
        let mut resultdummy: [u8;32]=[0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,1,2];
        
        let zkpbytes=zkp.r.to_bytes();
        let zkpsplitter=zkpbytes.split_at(32);
        resultdummy.clone_from_slice(zkpsplitter.0);
        resultbytes[0..32].clone_from_slice(zkpsplitter.0);
        //resultbytes[0..32]=resultdummy;
        //S bytes 
        let zkpbytes: k256::elliptic_curve::generic_array::GenericArray<u8, _>=zkp.s.to_bytes();
        let zkpsplitter=zkpbytes.split_at(32);
        resultdummy.clone_from_slice(zkpsplitter.0);
        resultbytes[32..64].clone_from_slice(zkpsplitter.0);
        let rbytes=bincode::serialize(&zkp.r).unwrap();
        resultbytes[0..32].clone_from_slice(&rbytes.as_ref());
        let sbytes=bincode::serialize(&zkp.s).unwrap();
        resultbytes[32..64].clone_from_slice(&sbytes.as_ref());
        /*
        let generic_array: [u32; 4] = [1, 2, 3, 4];
        let fixed_size_array: [u8; 16] = unsafe { std::mem::transmute(generic_array) };
        
         */
        //loop through 7 Commitments of 33 bytes 
        let mut commit_count=0;
        let mut startin_byte_index=64;
        while commit_count<7
        {   let ending_index=startin_byte_index+33;
            let commitmentbytes=commitments_party.commitments[commit_count].to_bytes();
            let commit_split=commitmentbytes.split_at(33);
            resultbytes[startin_byte_index..ending_index].clone_from_slice(commit_split.0);
            startin_byte_index=ending_index;
            commit_count=commit_count+1;
    
        }
        
        resultbytes[startin_byte_index..299].copy_from_slice(index.to_be_bytes().as_slice());
        
    
          
    
    
    
        resultbytes
    }

    fn convert_party3_to_bytes(index: &u32, commitments_party: &frost_secp256k1::Participant,zkp:&frost_secp256k1::nizk::NizkOfSecretKey) -> [u8;150]{



        let mut resultbytes:[u8;150]=[0;150];
        let mut resultdummy: [u8;40]=[0;40];
        println!("{}",zkp.r.to_bytes().len());
        let rbytes=bincode::serialize(&zkp.r).unwrap();
//        let  rkey: Result<Scalar, Box<bincode::ErrorKind>>  =bincode::deserialize(&rbytes.as_ref());
        
      //  println!("Party Rbytes for {} {:?}",index,rbytes); // for verification of Party R
      //  println!("Party R for {} {:?} after deserial",index,rkey.unwrap()); // for verification of Party R
        let split=rbytes.split_at(40);
        resultbytes[0..40].clone_from_slice(&split.0);
        //println!("Party S for {} {:?}",index,zkp.s); // for verification of Party S
        let sbytes=bincode::serialize(&zkp.s).unwrap();
        let split=sbytes.split_at(40);
        resultbytes[40..80].clone_from_slice(&split.0);
        let  skey: Result<Scalar, Box<bincode::ErrorKind>>  =bincode::deserialize(&sbytes.as_ref());
        
       // println!("Party S for {} {:?}",index,zkp.s); // for verification of Party R
       // println!("Party S for {}  after deserial{:?}",index,skey.unwrap()); // for verification of Party R
        
        //loop through 2 Commitments of 33 bytes 
        let mut commit_count=0;
        let mut startin_byte_index=80;
        while commit_count<2
        {   let ending_index=startin_byte_index+33;
            let commitmentbytes=commitments_party.commitments[commit_count].to_bytes();
            let commit_split=commitmentbytes.split_at(33);
            resultbytes[startin_byte_index..ending_index].clone_from_slice(commit_split.0);
            startin_byte_index=ending_index;
            commit_count=commit_count+1;
    
        }
        
        resultbytes[startin_byte_index..150].copy_from_slice(index.to_be_bytes().as_slice());
        
          
        
    
    
        resultbytes
    }

    fn convert_bytes_to_party3(party_bytes: &[u8;150]) -> (Participant)
    {
        let mut commit_vector:Vec<k256::ProjectivePoint>=vec!();
        
        let mut bytes_sequence :[u8;4]=[0,0,0,0];
        bytes_sequence.clone_from_slice(&party_bytes[146..150]);
        
        //let value_index=indexconvert as u32
        let index_u32_integer: u32 = ((bytes_sequence[0] as u32) << 24)
                        | ((bytes_sequence[1] as u32) << 16)
                        | ((bytes_sequence[2] as u32) << 8)
                        | (bytes_sequence[3] as u32);;
       
        let mut bytes_for_r: [u8;40]=[0;40];
        let mut bytes_for_s:[u8;40]=[0;40];
        bytes_for_r.copy_from_slice(&party_bytes[0..40]);
        bytes_for_s.copy_from_slice(&party_bytes[40..80]);
        // create scaler from deserializing bytes
        
        let  skey: Result<Scalar, Box<bincode::ErrorKind>>  =bincode::deserialize(bytes_for_s.as_ref());
        let  rkey: Result<Scalar, Box<bincode::ErrorKind>>  =bincode::deserialize(bytes_for_r.as_ref());
        
        // prepare zk proof from scaler values
        //let value =frost_secp256k1::nizk::NizkOfSecretKey{s,r}
        let  zkpfull :frost_secp256k1::nizk::NizkOfSecretKey= frost_secp256k1::nizk::NizkOfSecretKey { s: skey.unwrap(), r: rkey.unwrap() };
       
        let mut commit=0;
        let mut start_bytes=80;
        while(commit<2)
        {
            let endvalue=start_bytes+33;
            let mut bytescommit:[u8;33]=[0;33];                                   
            bytescommit.copy_from_slice(&party_bytes[start_bytes..endvalue]);
            let mut genarray=GenericArray::from_slice(bytescommit.as_ref());
            let mut byte_projective=k256::ProjectivePoint::from_bytes(&genarray).unwrap();                   
            commit_vector.push(byte_projective);
            start_bytes=endvalue;
            commit=commit+1;
    
        }
        //let mut poof :ZKPSecretKey;
        
        
        
    
        let  party_convert: Participant=Participant { index: index_u32_integer , commitments: commit_vector, proof_of_secret_key: zkpfull };
    
    
    party_convert
    }    

    fn convert_secret_to_3bytes(secretvector: &Vec<SecretShare>)->[u8;88]
{
    let total=secretvector.len();
    let mut count=0;
    let mut secretbytes: [u8;88]=[0;88];
    let mut startindex=0;
    let mut endindex=0;
    while count<total
    {   
        let writebytes: Vec<u8>=bincode::serialize(&secretvector[count]).unwrap();
        
        let size: usize =writebytes.len();
        endindex=endindex+size;
        secretbytes[startindex..endindex].copy_from_slice(writebytes.as_slice());

        //bytes_for_r.copy_from_slice(&party_bytes[0..40]);
        
        println!("{}",size);

        count=count+1;
        startindex=endindex;

    }
    

    secretbytes

}
fn convert_bytes_to_3secret(secretbytes:[u8;88] )->Vec<SecretShare>
{
    
    let mut secret_vector_from_bytes :Vec<SecretShare>=vec![];
    
     let mut startindex=0;
     let mut endindex=44;
     let mut total=3;
     let mut count=1;
     while count<total
    {   
        let mut bytesvalues: [u8;44]=[0;44];
        bytesvalues.copy_from_slice(&secretbytes[startindex..endindex]);
        let clone_secret_share: Result<SecretShare, Box<bincode::ErrorKind>>=bincode::deserialize(&bytesvalues);
        secret_vector_from_bytes.push(clone_secret_share.unwrap());
                count=count+1;
         startindex=endindex;
         endindex=endindex+44;

    }
    
    secret_vector_from_bytes

}

}

pub struct PublicCommitShareListformain {
    /// The participant's index.
    pub participant_index: u32,
    /// The published commitments.
    pub commitments: Vec<(AffinePoint, AffinePoint)>,
}

// impl PublicCommitShareListformain {
//     fn new() -> Self {
//         {

//         }
//     }
// }

fn public_commitment_to_bytes(publiccomitmentsharelist:&PublicCommitmentShareList )->[u8;70] 
{
    // Struct 33 +33 +4 =70bytes
    let mut returnbytes: [u8;70]=[0;70];
    returnbytes[0..33].copy_from_slice(&publiccomitmentsharelist.commitments[0].0.to_bytes());
    returnbytes[33..66].copy_from_slice(&publiccomitmentsharelist.commitments[0].1.to_bytes());
    returnbytes[66..70].copy_from_slice(&publiccomitmentsharelist.participant_index.to_be_bytes());
    
    returnbytes    

}

fn public_bytes_to_commitment(returnbytes:[u8;70] )->PublicCommitmentShareList
{
    
    //let mut bytesvalue: [u8;70]=[0;70];
    let mut indexbytes:[u8;4]=[0;4];
    indexbytes.copy_from_slice(&returnbytes[66..70]);
    let indexcommit:u32=u32::from_be_bytes(indexbytes);
    
    // let (mut p1_public_comshares, mut p1_secret_comshares) = generate_commitment_share_lists(&mut OsRng, indexcommit, 1);
    
    

    
     let mut affinebytes:[u8;33]=[0;33];
     affinebytes.copy_from_slice(&returnbytes[0..33]);
     let mut genarrya=GenericArray::from_slice(affinebytes.as_ref());
     let affine1 :AffinePoint=AffinePoint::from_bytes(&genarrya).unwrap();
     
     affinebytes.copy_from_slice(&returnbytes[33..66]);
     let mut genarrya=GenericArray::from_slice(affinebytes.as_ref());
     let affine2 :AffinePoint=AffinePoint::from_bytes(&genarrya).unwrap();
     //let mut p1_public_comshares:PublicCommitShareList=PublicCommitmentShareList

     let mut tuple: (AffinePoint, AffinePoint)=(affine1,affine2);
     tuple.0.clone_from(&affine1);
     tuple.1.clone_from(&affine2);

     let mut vec_of_tuples = Vec::new();
     vec_of_tuples.push(tuple);


     //let mut points: Vec<AffinePoint> = Vec::new();
     
     
     //points.push(affine1);
     //points.push(affine2);
     
     //let Commitment :CommitmentShare;
     
     let p1_public_comshares=PublicCommitmentShareList { participant_index: indexcommit,commitments:vec_of_tuples  };
     //p1_public_comshares.commitments[0].0.clone_from(&affine1);
     //p1_public_comshares.commitments[0].1.clone_from(&affine2);


    // let affine1 : AffinePoint=AffinePoint::from_bytes(&affinebytes.as_ref()).unwrap();
    // p1_public_comshares.commitments[0].0.clone_from(source)
    // p1_public_comshares.commitments[0].1.clone_from(source)
   // bytesvalue.clone_from_slice(&returnbytes[0..33]);

    //let mut returncommit: PublicCommitmentShareList=
    //returncommit.participant_index=1;
    //let mut genarray=GenericArray::from_slice(bytesvalue.as_ref());
   // let mut returncommit: PublicCommitmentShareList=PublicCommitShareList::into(self)
    //let mut genarray=GenericArray::from_slice(bytescommit.as_ref());
    //let affine1=AffinePoint::from_bytes(&returnbytes[0..33]).unwrap();
p1_public_comshares

}
fn signer_vector_tobytes(signers: &Vec<frost_secp256k1::signature::Signer>, indexsign: u32)->[u8;140] 
{
    // for two signers
    let mut index=indexsign;
    let mut returnbytes: [u8;140]=[0;140];
    let bytes1=signers[index as usize].published_commitment_share.0.to_bytes();
    let bytes2=signers[index as usize].published_commitment_share.1.to_bytes();
    returnbytes[0..33].copy_from_slice(&bytes1);
    returnbytes[33..66].copy_from_slice(&bytes2);
    returnbytes[66..70].copy_from_slice(&signers[index as usize].participant_index.to_be_bytes());

    //copy second signer in memory 
    index=index+1;
    //
    let bytes1=signers[index as usize].published_commitment_share.0.to_bytes();
    let bytes2=signers[index as usize].published_commitment_share.1.to_bytes();
    returnbytes[70..103].copy_from_slice(&bytes1);
    returnbytes[103..136].copy_from_slice(&bytes2);
    returnbytes[136..140].copy_from_slice(&signers[index as usize].participant_index.to_be_bytes());



    // returnbytes[0..33].copy_from_slice(&publiccomitmentsharelist.commitments[0].0.to_bytes());
    // returnbytes[33..66].copy_from_slice(&publiccomitmentsharelist.commitments[0].1.to_bytes());
    // returnbytes[66..70].copy_from_slice(&publiccomitmentsharelist.participant_index.to_be_bytes());
    
    returnbytes    
    //let firstbytes=public_commitment_to_bytes(&signers[0].published_commitment_share);
    
    

}
fn signer_bytes_tovector( signerbytes:[u8;140] )-> Vec<frost_secp256k1::signature::Signer>
{
    // for two signers
    let mut signervector :Vec<frost_secp256k1::signature::Signer>=vec![];
    let mut indexbytes:[u8;4]=[0;4];
    indexbytes.copy_from_slice(&signerbytes[66..70]);
    let indexcommit:u32=u32::from_be_bytes(indexbytes);

    
    //signervector[0].participant_index=indexcommit;
    //signervector[0].published_commitment_share

    let mut affinebytes:[u8;33]=[0;33];
     affinebytes.copy_from_slice(&signerbytes[0..33]);
     let  genarrya=GenericArray::from_slice(affinebytes.as_ref());
     let affine1 :AffinePoint=AffinePoint::from_bytes(&genarrya).unwrap();
     
     affinebytes.copy_from_slice(&signerbytes[33..66]);
     let  genarrya=GenericArray::from_slice(affinebytes.as_ref());
     let affine2 :AffinePoint=AffinePoint::from_bytes(&genarrya).unwrap();
     // copy affines converted from memory bytes to back 
     //signervector[0].published_commitment_share.0.clone_from(&affine1);
     //signervector[0].published_commitment_share.1.clone_from(&affine2);
     let mut signer1:frost_secp256k1::signature::Signer=Signer { participant_index: indexcommit, published_commitment_share: (affine1,affine2) };
    //signer1.participant_index=indexcommit;
    //signer1.published_commitment_share.0.clone_from(&affine1);
    //signer1.published_commitment_share.0.clone_from(&affine2);
    signervector.push(signer1);

// Convert back second signers 

    indexbytes.copy_from_slice(&signerbytes[136..140]);
    let indexcommit:u32=u32::from_be_bytes(indexbytes);

    
    //signervector[1].participant_index=indexcommit;
    //signervector[0].published_commitment_share

    let mut affinebytes:[u8;33]=[0;33];
     affinebytes.copy_from_slice(&signerbytes[70..103]);
     let genarrya=GenericArray::from_slice(affinebytes.as_ref());
     let affine1 :AffinePoint=AffinePoint::from_bytes(&genarrya).unwrap();
     
     affinebytes.copy_from_slice(&signerbytes[103..136]);
     let  genarrya=GenericArray::from_slice(affinebytes.as_ref());
     let affine2 :AffinePoint=AffinePoint::from_bytes(&genarrya).unwrap();
     // copy affines converted from memory bytes to back 
     //signervector[1].published_commitment_share.0.clone_from(&affine1);
     //signervector[1].published_commitment_share.1.clone_from(&affine2);

     let mut signer2:frost_secp256k1::signature::Signer=Signer { participant_index: indexcommit, published_commitment_share: (affine1,affine2) };
    // signer2.participant_index=indexcommit;
    // signer2.published_commitment_share.0.clone_from(&affine1);
    // signer2.published_commitment_share.0.clone_from(&affine2);
    signervector.push(signer2);
    return signervector;

    

}

// fn public_bytes_to_commitment2(returnbytes:[u8;70] )->PublicCommitShareListformain
// {
    
//     //let mut bytesvalue: [u8;70]=[0;70];
//     let mut indexbytes:[u8;4]=[0;4];
//     indexbytes.copy_from_slice(&returnbytes[66..70]);
//     let indexcommit:u32=u32::from_be_bytes(indexbytes);
//     let mut p1_public_comshares:PublicCommitShareListformain;
//     // let (mut p1_public_comshares, mut p1_secret_comshares) = generate_commitment_share_lists(&mut OsRng, indexcommit, 1);
//     /*
    
    

    
//      let mut affinebytes:[u8;33]=[0;33];
//      affinebytes.copy_from_slice(&returnbytes[0..33]);
//      let mut genarrya=GenericArray::from_slice(affinebytes.as_ref());
//      let affine1 :AffinePoint=AffinePoint::from_bytes(&genarrya).unwrap();
     
//      affinebytes.copy_from_slice(&returnbytes[33..66]);
//      let mut genarrya=GenericArray::from_slice(affinebytes.as_ref());
//      let affine2 :AffinePoint=AffinePoint::from_bytes(&genarrya).unwrap();
//      p1_public_comshares.commitments[0].0.clone_from(&affine1);
//      p1_public_comshares.commitments[0].1.clone_from(&affine2);
//     // let affine1 : AffinePoint=AffinePoint::from_bytes(&affinebytes.as_ref()).unwrap();
//     // p1_public_comshares.commitments[0].0.clone_from(source)
//     // p1_public_comshares.commitments[0].1.clone_from(source)
//    // bytesvalue.clone_from_slice(&returnbytes[0..33]);

//     //let mut returncommit: PublicCommitmentShareList=
//     //returncommit.participant_index=1;
//     //let mut genarray=GenericArray::from_slice(bytesvalue.as_ref());
//    // let mut returncommit: PublicCommitmentShareList=PublicCommitShareList::into(self)
//     //let mut genarray=GenericArray::from_slice(bytescommit.as_ref());
//     //let affine1=AffinePoint::from_bytes(&returnbytes[0..33]).unwrap();
//      */
// p1_public_comshares

//}

// impl Handler {
//     pub async fn new(config: config::ChainConfig) -> Self {
//         let provider = Provider::try_from(config.node).unwrap_or_else(|e| {
//             panic!(
//                 "Error getting ethers provider {e:#?} with node {:#?}",
//                 config.node
//             )
//         });

//         let wallet = Wallet::from_str(config.private_key)
//             .unwrap_or_else(|e| panic!("Error setting wallet {e:#?} using private key"));

//             let wallet = wallet.with_chain_id(config.chain_nonce);

//         let address: EthersH160 = config.minter.parse().unwrap_or_else(|e| {
//             panic!(
//                 "Error parsing minter address with value {:#?} with error {:#?}",
//                 config.minter, e
//             )
//         });

//         let signer = SignerMiddleware::new(provider.clone(), wallet);
//         let minter = MinterContract::new(address, signer.clone().into());

//         Handler {
//             config,
//             signer,
//             minter,
//             provider,
//             log_hashes: Arc::new(RwLock::new(HashMap::new())),
//         }
//     }
// }