use emurgo_message_signing as ms;
use cardano_serialization_lib as csl;

fn main() {
    use ms::utils::ToBytes;
    // 1) Create some arbitrary keys/message just so we can run this example
    let sk_bytes: [u8; 32] = [
         34, 125,  55,  10, 222, 244,  31,  91, 181, 231,  62,  80,  90,  53, 246, 160,
        226, 111, 123, 228, 188,  90,  15, 130, 210, 206,  78, 199, 209,  18, 202, 234
    ];
    let sk = csl::crypto::PrivateKey::from_normal_bytes(&sk_bytes).unwrap();
    let pk = sk.to_public();
    let payload = "message to sign".as_bytes().to_vec();
    // We can also optionally supply external data which is not included
    // in the final signed object, but is signed to form the signature.
    // See section 4.3 of the COSE RFC 8152
    let external_aad = "externally supplied data not in sign object".as_bytes().to_vec();

    // 2) Creating a simple signed message
    // protected headers are those that are actually signed
    let mut protected_headers = ms::HeaderMap::new();
    let protected_serialized = ms::ProtectedHeaderMap::new(&protected_headers);
    let unprotected = ms::HeaderMap::new();
    let headers = ms::Headers::new(&protected_serialized, &unprotected);
    // we will use COSESign1Builder to simplify the whole proces and make it less error-prone
    // if it is not ued then the appropriate SigStucture must be manually created and 
    let mut builder = ms::builders::COSESign1Builder::new(&headers, payload, false);
    // since we have external data we must set it here so the signature matches.
    // omit this step if your use-case doesn't use it
    builder.set_external_aad(external_aad.clone());
    // remember that we sign SigStructure, not the message/headres itself.
    let to_sign = builder.make_data_to_sign().to_bytes();
    // it is important that we sign using Ed25519 keys in accordance to the spec
    // make sure your key is not in the X25519 format.
    let signed_sig_struct = sk.sign(&to_sign).to_bytes();
    // then once we have signed that we can build the final COSESign1 result to be shared
    let cose_sign1 = builder.build(signed_sig_struct);

    // 3) The recipient can then verify the message back
    // Any user here should carefully inspect the headers / payload
    // to make sure they are verifying the correct sign object
    // (not shown here for simplicity)
    let payload_to_verify = cose_sign1.payload();
    let headers_to_verify = cose_sign1.headers();
    // Reconstruct the SigStructure object so we can verify the signature
    let sig_struct_reconstructed = cose_sign1.signed_data(Some(external_aad), None).unwrap().to_bytes();
    let sig = csl::crypto::Ed25519Signature::from_bytes(cose_sign1.signature()).unwrap();
    assert!(pk.verify(&sig_struct_reconstructed, &sig));
}