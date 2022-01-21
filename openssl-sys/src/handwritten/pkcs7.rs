use libc::*;
use *;

// use x509::stack_st_X509;
// use x509_attr::stack_st_X509_ATTRIBUTE;

stack!(stack_st_PKCS7_SIGNER_INFO);
stack!(stack_st_PKCS7_RECIP_INFO);

// TODO bk add other ossl versions?
cfg_if! {
    if #[cfg(any(ossl111))] {
        #[repr(C)]
        pub struct PKCS7_SIGNED {
            pub version: *mut ASN1_INTEGER, /* version 1 */
            pub md_algs: *mut stack_st_X509_ALGOR, /* md used */
            pub cert: *mut stack_st_X509, /* [ 0 ] */
            pub crl: *mut stack_st_X509_CRL, /* [ 1 ] */
            pub signer_info: *mut stack_st_PKCS7_SIGNER_INFO,
            pub contents: *mut PKCS7,
        }
    } else {
        pub enum PKCS7_SIGNED {}
    }
}

// TODO bk add other ossl versions?
cfg_if! {
    if #[cfg(any(ossl111))] {
        #[repr(C)]
        pub struct PKCS7_ENC_CONTENT {
            pub content_type: *mut ASN1_OBJECT,
            pub algorithm: *mut X509_ALGOR,
            pub enc_data: *mut ASN1_OCTET_STRING, /* [ 0 ] */
            pub cipher: *const EVP_CIPHER,
        }
    } else {
        pub enum PKCS7_ENC_CONTENT {}
    }
}

// TODO bk add other ossl versions?
cfg_if! {
    if #[cfg(any(ossl111))] {
        #[repr(C)]
        pub struct PKCS7_ENVELOPE {
            pub version: *mut ASN1_INTEGER, /* version 0 */
            pub recipientinfo: *mut stack_st_PKCS7_RECIP_INFO,
            pub enc_data: *mut PKCS7_ENC_CONTENT,
        }
    }  else {
        pub enum PKCS7_ENVELOPE {}
    }
}

// TODO bk add other ossl versions?
cfg_if! {
    if #[cfg(any(ossl111))] {
        #[repr(C)]
        pub struct PKCS7_SIGN_ENVELOPE {
            pub version: *mut ASN1_INTEGER, /* version 1 */
            pub md_algs: *mut stack_st_X509_ALGOR, /* md used */
            pub cert: *mut stack_st_X509, /* [ 0 ] */
            pub crl: *mut stack_st_X509_CRL, /* [ 1 ] */
            pub signer_info: *mut stack_st_PKCS7_SIGNER_INFO,
            pub enc_data: *mut PKCS7_ENC_CONTENT,
            pub recipientinfo: *mut stack_st_PKCS7_RECIP_INFO
        }
    } else {
        pub enum PKCS7_SIGN_ENVELOPE {}
    }
}

// TODO bk add other ossl versions?
cfg_if! {
    if #[cfg(any(ossl111))] {
        #[repr(C)]
        pub struct PKCS7_DIGEST {
            pub version: *mut ASN1_INTEGER, /* version 0 */
            pub md: *mut X509_ALGOR, /* md used */
            pub contents: *mut PKCS7,
            pub digest: *mut ASN1_OCTET_STRING,
        }
    } else {
        pub enum PKCS7_DIGEST {}
    }
}

// TODO bk add other ossl versions?
cfg_if! {
    if #[cfg(any(ossl111))] {
        #[repr(C)]
        pub struct PKCS7_ENCRYPT {
            pub version: *mut ASN1_INTEGER, /* version 0 */
            pub enc_data: *mut PKCS7_ENC_CONTENT,
        }
    } else {
        pub enum PKCS7_ENCRYPT {}
    }
}

extern "C" {
    pub fn PKCS7_SIGNED_free(info: *mut PKCS7_SIGNED);
    pub fn PKCS7_ENC_CONTENT_free(info: *mut PKCS7_ENC_CONTENT);
    pub fn PKCS7_ENVELOPE_free(info: *mut PKCS7_ENVELOPE);
    pub fn PKCS7_SIGN_ENVELOPE_free(info: *mut PKCS7_SIGN_ENVELOPE);
    pub fn PKCS7_DIGEST_free(info: *mut PKCS7_DIGEST);
    pub fn PKCS7_SIGNER_INFO_free(info: *mut PKCS7_SIGNER_INFO);
}

pub enum PKCS7 {}

// TODO bk remove, as union not supported?
// TODO bk add other ossl versions?
// cfg_if! {
//     if #[cfg(any(ossl111))] {
//         #[repr(C)]
//         pub struct PKCS7 {
//             /*
//              * The following is non NULL if it contains ASN1 encoding of this
//              * structure
//              */
//             pub asn1: *mut c_uchar,
//             pub length: c_long,
//             // # define PKCS7_S_HEADER  0
//             // # define PKCS7_S_BODY    1
//             // # define PKCS7_S_TAIL    2
//             pub state: c_int, /* used during processing */
//             pub detached: c_int,
//             pub r#type: *mut ASN1_OBJECT,
//             /* content as defined by the type */
//             /*
//              * all encryption/message digests are applied to the 'contents', leaving
//              * out the 'type' field.
//              */
//             pub d: *mut PKCS7_SIGNED,
//             // pub d: union {
//             //     pub ptr: *mut c_char,
//             //     /* NID_pkcs7_data */
//             //     pub data: *mut ASN1_OCTET_STRING,
//             //     /* NID_pkcs7_signed */
//             //     pub sign: *mut PKCS7_SIGNED,
//             //     /* NID_pkcs7_enveloped */
//             //     pub enveloped: *mut PKCS7_ENVELOPE,
//             //     /* NID_pkcs7_signedAndEnveloped */
//             //     pub signed_and_enveloped: *mut PKCS7_SIGN_ENVELOPE,
//             //     /* NID_pkcs7_digest */
//             //     pub digest: *mut PKCS7_DIGEST,
//             //     /* NID_pkcs7_encrypted */
//             //     pub encrypted: *mut PKCS7_ENCRYPT,
//             //     /* Anything else */
//             //     pub other: *mut ASN1_TYPE,
//             // },
//         }
//     } else {
//          pub enum PKCS7 {}
//     }
// }

// TODO bk add other ossl versions?
cfg_if! {
    if #[cfg(any(ossl111))] {
        #[repr(C)]
        pub struct PKCS7_ISSUER_AND_SERIAL {
            pub issuer: *mut X509_NAME,
            pub serial: *mut ASN1_INTEGER,
        }
    } else {
        pub enum PKCS7_ISSUER_AND_SERIAL {}
    }
}

cfg_if! {
    if #[cfg(any(ossl111))] {
        #[repr(C)]
        pub struct PKCS7_SIGNER_INFO {
            pub version: *mut ASN1_INTEGER, /* version 1 */
            pub issuer_and_serial: *mut PKCS7_ISSUER_AND_SERIAL,
            pub digest_alg: *mut X509_ALGOR,
            pub auth_attr: *mut stack_st_X509_ATTRIBUTE, /* [ 0 ] */
            pub digest_enc_alg: *mut X509_ALGOR,
            pub enc_digest: *mut ASN1_OCTET_STRING,
            pub unauth_attr: *mut stack_st_X509_ATTRIBUTE, /* [ 1 ] */
            pub pkey: *mut EVP_PKEY, /* The private key to sign with */
        }
    } else {
        pub enum PKCS7_SIGNER_INFO {}
    }
}

extern "C" {
    pub fn d2i_PKCS7(a: *mut *mut PKCS7, pp: *mut *const c_uchar, length: c_long) -> *mut PKCS7;
}

const_ptr_api! {
    extern "C" {
        pub fn i2d_PKCS7(a: #[const_ptr_if(ossl300)] PKCS7, buf: *mut *mut u8) -> c_int;
    }
}

extern "C" {
    pub fn PKCS7_encrypt(
        certs: *mut stack_st_X509,
        b: *mut BIO,
        cipher: *const EVP_CIPHER,
        flags: c_int,
    ) -> *mut PKCS7;

    pub fn PKCS7_verify(
        pkcs7: *mut PKCS7,
        certs: *mut stack_st_X509,
        store: *mut X509_STORE,
        indata: *mut BIO,
        out: *mut BIO,
        flags: c_int,
    ) -> c_int;

    pub fn PKCS7_get0_signers(
        pkcs7: *mut PKCS7,
        certs: *mut stack_st_X509,
        flags: c_int,
    ) -> *mut stack_st_X509;

    pub fn PKCS7_sign(
        signcert: *mut X509,
        pkey: *mut EVP_PKEY,
        certs: *mut stack_st_X509,
        data: *mut BIO,
        flags: c_int,
    ) -> *mut PKCS7;

    pub fn PKCS7_decrypt(
        pkcs7: *mut PKCS7,
        pkey: *mut EVP_PKEY,
        cert: *mut X509,
        data: *mut BIO,
        flags: c_int,
    ) -> c_int;

    pub fn PKCS7_free(pkcs7: *mut PKCS7);

    pub fn SMIME_write_PKCS7(
        out: *mut BIO,
        pkcs7: *mut PKCS7,
        data: *mut BIO,
        flags: c_int,
    ) -> c_int;

    pub fn SMIME_read_PKCS7(bio: *mut BIO, bcont: *mut *mut BIO) -> *mut PKCS7;

    pub fn PKCS7_new() -> *mut PKCS7;

    pub fn PKCS7_set_type(p7: *mut PKCS7, nid_pkcs7: c_int) -> c_int;

    pub fn PKCS7_add_certificate(p7: *mut PKCS7, x509: *mut X509) -> c_int;

    pub fn PKCS7_add_signature(
        p7: *mut PKCS7,
        x509: *mut X509,
        pkey: *mut EVP_PKEY,
        digest: *const EVP_MD
    ) -> *mut PKCS7_SIGNER_INFO;

    pub fn PKCS7_set_signed_attributes(
        p7si: *mut PKCS7_SIGNER_INFO,
        attributes: *mut stack_st_X509_ATTRIBUTE
    ) -> c_int;

    pub fn PKCS7_add_signed_attribute(
        p7si: *mut PKCS7_SIGNER_INFO,
        nid: c_int,
        attrtype: c_int,
        data: *mut c_void
    ) -> c_int;

    pub fn PKCS7_content_new(p7: *mut PKCS7, nid_pkcs7: c_int) -> c_int;

    pub fn PKCS7_dataInit(p7: *mut PKCS7, bio: *mut BIO) -> *mut BIO;

    pub fn PKCS7_dataFinal(p7: *mut PKCS7, bio: *mut BIO) -> c_int;

    pub fn i2d_PKCS7_bio(bio: *mut BIO, p7: *mut PKCS7) -> c_int;
}
