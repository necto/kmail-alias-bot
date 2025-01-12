(use-modules (guix packages)
             (guix git-download)
             (guix download)
             (guix gexp)
             (guix build-system cargo)
             (guix utils)
             (gnu packages certs) ; for nss-certs
             (gnu packages compression) ; for zstd
             (gnu packages rust)
             (gnu packages crates-database)
             (gnu packages crates-io)
             (gnu packages crates-web)
             (gnu packages crates-tls)
             (gnu packages crates-crypto)
             (gnu packages crates-compression)
             (gnu packages crates-check)
             (gnu packages crates-windows)
             (gnu packages pkg-config)
             (gnu packages tls) ; for openssl
             ((guix licenses) #:prefix license:))

(define-public rust-teloxide-tests-macros-0.2
  (package
    (name "rust-teloxide-tests-macros")
    (version "0.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "teloxide_tests_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0nh9a86fln23ymm2xys70cssspya5vdpjan60pbxy2pxnhwcassa"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "")
    (synopsis "Proc macros for teloxide_tests")
    (description "This package provides Proc macros for teloxide_tests.")
    (license license:expat)))

(define-public rust-mime-guess-2
  (package
    (name "rust-mime-guess")
    (version "2.0.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mime_guess" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "03jmg3yx6j39mg0kayf7w4a886dl3j15y8zs119zw01ccy74zi7p"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-mime" ,rust-mime-0.3)
                       ("rust-unicase" ,rust-unicase-2)
                       ("rust-unicase" ,rust-unicase-2))
       #:cargo-development-inputs (("rust-criterion" ,rust-criterion-0.3))))
    (home-page "https://github.com/abonander/mime_guess")
    (synopsis
     "simple crate for detection of a file's MIME type by its extension.")
    (description
     "This package provides a simple crate for detection of a file's MIME type by its
extension.")
    (license license:expat)))

(define-public rust-gag-1
  (package
    (name "rust-gag")
    (version "1.0.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "gag" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0wjr02svx7jir7b7r69lpfh3assasmqsz4vivzzzpsb677hvw4x7"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-filedescriptor" ,rust-filedescriptor-0.8)
                       ("rust-tempfile" ,rust-tempfile-3))
       #:cargo-development-inputs (("rust-dirs" ,rust-dirs-3)
                                   ("rust-lazy-static" ,rust-lazy-static-1))))
    (home-page "https://github.com/Stebalien/gag-rs")
    (synopsis
     "Gag, redirect, or hold stdout/stderr output. Currently only *nix operating systems are supported")
    (description
     "This package provides Gag, redirect, or hold stdout/stderr output.  Currently only *nix operating
systems are supported.")
    (license license:expat)))

(define-public rust-serde-cbor-2-0.12
  (package
    (name "rust-serde-cbor-2")
    (version "0.12.0-dev")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "serde_cbor_2" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0mfxl2b08f5w1xyq7740kf4vvyqnsqrg804vpvfiw7z097s7avdl"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-half" ,rust-half-1)
                       ("rust-serde" ,rust-serde-1))
       #:cargo-development-inputs (("rust-serde-derive" ,rust-serde-derive-1))))
    (home-page "https://github.com/kanidm/cbor")
    (synopsis "CBOR support for serde")
    (description "This package provides CBOR support for serde.")
    (license (list license:expat license:asl2.0))))

(define-public rust-mediatype-0.19
  (package
    (name "rust-mediatype")
    (version "0.19.18")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mediatype" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1pbjxbagyp52vzxsg6d533v95wbw5djkc2msnbj8m31w3f6wsy48"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-serde" ,rust-serde-1))
       #:cargo-development-inputs (("rust-serde-json" ,rust-serde-json-1))))
    (home-page "https://github.com/picoHz/mediatype")
    (synopsis "MIME Media-type parsing")
    (description "This package provides MIME Media-type parsing.")
    (license license:expat)))

(define-public rust-actix-web-lab-derive-0.20
  (package
    (name "rust-actix-web-lab-derive")
    (version "0.20.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "actix-web-lab-derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0s79npywpc69a1rziskpsxwmppg8a5abp7gjj6v7cjnyr23v584s"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/robjtede/actix-web-lab")
    (synopsis "Experimental macros for Actix Web")
    (description "This package provides Experimental macros for Actix Web.")
    (license (list license:expat license:asl2.0))))

(define-public rust-actix-web-lab-0.20
  (package
    (name "rust-actix-web-lab")
    (version "0.20.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "actix-web-lab" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1iikdzc7vj4nza96w0l49679sh1yir4aipjchjf1f6zc9slc2xbn"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-actix-files" ,rust-actix-files-0.6)
                       ("rust-actix-http" ,rust-actix-http-3)
                       ("rust-actix-router" ,rust-actix-router-0.5)
                       ("rust-actix-service" ,rust-actix-service-2)
                       ("rust-actix-utils" ,rust-actix-utils-3)
                       ("rust-actix-web" ,rust-actix-web-4)
                       ("rust-actix-web-lab-derive" ,rust-actix-web-lab-derive-0.20)
                       ("rust-ahash" ,rust-ahash-0.8)
                       ("rust-arc-swap" ,rust-arc-swap-1)
                       ("rust-async-trait" ,rust-async-trait-0.1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-bytestring" ,rust-bytestring-1)
                       ("rust-csv" ,rust-csv-1)
                       ("rust-derive-more" ,rust-derive-more-0.99)
                       ("rust-futures-core" ,rust-futures-core-0.3)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-http" ,rust-http-0.2)
                       ("rust-impl-more" ,rust-impl-more-0.1)
                       ("rust-itertools" ,rust-itertools-0.12)
                       ("rust-local-channel" ,rust-local-channel-0.1)
                       ("rust-mediatype" ,rust-mediatype-0.19)
                       ("rust-mime" ,rust-mime-0.3)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-pin-project-lite" ,rust-pin-project-lite-0.2)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-rmp-serde" ,rust-rmp-serde-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-cbor-2" ,rust-serde-cbor-2-0.12)
                       ("rust-serde-html-form" ,rust-serde-html-form-0.2)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-stream" ,rust-tokio-stream-0.1)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/robjtede/actix-web-lab")
    (synopsis "In-progress extractors and middleware for Actix Web")
    (description
     "This package provides In-progress extractors and middleware for Actix Web.")
    (license (list license:expat license:asl2.0))))

(define-public rust-common-multipart-rfc7578-0.6
  (package
    (name "rust-common-multipart-rfc7578")
    (version "0.6.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "common-multipart-rfc7578" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0sc8g59a6xkli52a74mx94dw84f1sznf30v5yaq6afb0phkf7bjv"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-bytes" ,rust-bytes-1)
                       ("rust-futures-core" ,rust-futures-core-0.3)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-http" ,rust-http-0.2)
                       ("rust-mime" ,rust-mime-0.3)
                       ("rust-mime-guess" ,rust-mime-guess-2)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-thiserror" ,rust-thiserror-1))
       #:cargo-development-inputs (("rust-futures-util" ,rust-futures-util-0.3)
                                   ("rust-hyper" ,rust-hyper-0.14)
                                   ("rust-tokio" ,rust-tokio-1))))
    (home-page "https://github.com/ferristseng/rust-multipart-rfc7578")
    (synopsis "An implementation of multipart/form-data (RFC7578)")
    (description
     "This package provides An implementation of multipart/form-data (RFC7578).")
    (license (list license:expat license:asl2.0))))

(define-public rust-actix-multipart-rfc7578-0.10
  (package
    (name "rust-actix-multipart-rfc7578")
    (version "0.10.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "actix-multipart-rfc7578" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0mwjw4q3zqjmny781b383wvavsawymfidf38sh49ncx6kin2fybv"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-actix-http" ,rust-actix-http-3)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-common-multipart-rfc7578" ,rust-common-multipart-rfc7578-0.6)
                       ("rust-futures-core" ,rust-futures-core-0.3)
                       ("rust-thiserror" ,rust-thiserror-1))
       #:cargo-development-inputs (("rust-actix-rt" ,rust-actix-rt-2)
                                   ("rust-awc" ,rust-awc-3))))
    (home-page "https://github.com/ferristseng/rust-multipart-rfc7578")
    (synopsis "An implementation of multipart/form-data (RFC7578) for Actix")
    (description
     "This package provides An implementation of multipart/form-data (RFC7578) for Actix.")
    (license (list license:expat license:asl2.0))))

(define-public rust-serde-plain-1
  (package
    (name "rust-serde-plain")
    (version "1.0.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "serde_plain" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0l4d4nbw00pz6n43icrc605bhgynfmlyq39sn8i10qasnrnzrqcw"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-serde" ,rust-serde-1))
       #:cargo-development-inputs (("rust-serde-derive" ,rust-serde-derive-1))))
    (home-page "https://docs.rs/serde_plain")
    (synopsis "restricted plain text serializer for serde")
    (description
     "This package provides a restricted plain text serializer for serde.")
    (license (list license:expat license:asl2.0))))

(define-public rust-parse-size-1
  (package
    (name "rust-parse-size")
    (version "1.1.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "parse-size" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "16xqfg6sm8p9l4hr0rf8skx1rx2saa4mr9mkz8dqrkhp3v6jqzs8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-development-inputs (("rust-clap" ,rust-clap-4))))
    (home-page "https://github.com/kennytm/parse-size")
    (synopsis "Parse byte size into integer accurately")
    (description
     "This package provides Parse byte size into integer accurately.")
    (license license:expat)))

(define-public rust-actix-multipart-derive-0.7
  (package
    (name "rust-actix-multipart-derive")
    (version "0.7.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "actix-multipart-derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0sqw9960yf83gi9aildrlapn3010nfp3v9rgx9w0cw4syi3vh7p1"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-darling" ,rust-darling-0.20)
                       ("rust-parse-size" ,rust-parse-size-1)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))
       #:cargo-development-inputs (("rust-actix-multipart" ,rust-actix-multipart-0.7)
                                   ("rust-actix-web" ,rust-actix-web-4)
                                   ("rust-rustversion" ,rust-rustversion-1)
                                   ("rust-trybuild" ,rust-trybuild-1))))
    (home-page "https://actix.rs")
    (synopsis "Multipart form derive macro for Actix Web")
    (description
     "This package provides Multipart form derive macro for Actix Web.")
    (license (list license:expat license:asl2.0))))

(define-public rust-actix-multipart-0.7
  (package
    (name "rust-actix-multipart")
    (version "0.7.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "actix-multipart" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0lqay0cf904sshq0wg7hig0y82h8wl7am1by9y4lxqz7vqk8l4fm"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-actix-multipart-derive" ,rust-actix-multipart-derive-0.7)
                       ("rust-actix-utils" ,rust-actix-utils-3)
                       ("rust-actix-web" ,rust-actix-web-4)
                       ("rust-derive-more" ,rust-derive-more-0.99)
                       ("rust-futures-core" ,rust-futures-core-0.3)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-httparse" ,rust-httparse-1)
                       ("rust-local-waker" ,rust-local-waker-0.1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-memchr" ,rust-memchr-2)
                       ("rust-mime" ,rust-mime-0.3)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-serde-plain" ,rust-serde-plain-1)
                       ("rust-tempfile" ,rust-tempfile-3)
                       ("rust-tokio" ,rust-tokio-1))
       #:cargo-development-inputs (("rust-actix-http" ,rust-actix-http-3)
                                   ("rust-actix-multipart-rfc7578" ,rust-actix-multipart-rfc7578-0.10)
                                   ("rust-actix-rt" ,rust-actix-rt-2)
                                   ("rust-actix-test" ,rust-actix-test-0.1)
                                   ("rust-actix-web" ,rust-actix-web-4)
                                   ("rust-assert-matches" ,rust-assert-matches-1)
                                   ("rust-awc" ,rust-awc-3)
                                   ("rust-env-logger" ,rust-env-logger-0.11)
                                   ("rust-futures-test" ,rust-futures-test-0.3)
                                   ("rust-futures-util" ,rust-futures-util-0.3)
                                   ("rust-multer" ,rust-multer-3)
                                   ("rust-tokio" ,rust-tokio-1)
                                   ("rust-tokio-stream" ,rust-tokio-stream-0.1))))
    (home-page "https://actix.rs")
    (synopsis "Multipart form support for Actix Web")
    (description "This package provides Multipart form support for Actix Web.")
    (license (list license:expat license:asl2.0))))

(define-public rust-teloxide-tests-0.2
  (package
    (name "rust-teloxide-tests")
    (version "0.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "teloxide_tests" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1ahgpi3wh3h2d7mhw9g693yzx75j4js20qx10yzc3cglf217sgh6"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-actix-multipart" ,rust-actix-multipart-0.7)
                       ("rust-actix-web" ,rust-actix-web-4)
                       ("rust-actix-web-lab" ,rust-actix-web-lab-0.20)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-ctrlc" ,rust-ctrlc-3)
                       ("rust-dotenv" ,rust-dotenv-0.15)
                       ("rust-env-logger" ,rust-env-logger-0.10)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-gag" ,rust-gag-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-mime" ,rust-mime-0.3)
                       ("rust-mime-guess" ,rust-mime-guess-2)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-pretty-env-logger" ,rust-pretty-env-logger-0.5)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-reqwest" ,rust-reqwest-0.12)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-teloxide" ,rust-teloxide-0.13)
                       ("rust-teloxide-tests-macros" ,rust-teloxide-tests-macros-0.2)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-url" ,rust-url-2))
       #:cargo-development-inputs (("rust-serial-test" ,rust-serial-test-3))))
    (home-page "https://github.com/LasterAlex/teloxide_tests")
    (synopsis "Test suite for teloxide bots")
    (description "This package provides Test suite for teloxide bots.")
    (license license:expat)))

(define-public rust-teloxide-macros-0.8
  (package
    (name "rust-teloxide-macros")
    (version "0.8.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "teloxide-macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1h8qq0p0nhy9saj8z9zh2i8288c8z7fyv2xik8d1dry317c36bby"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-heck" ,rust-heck-0.4)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/teloxide/teloxide")
    (synopsis "The teloxide's procedural macros")
    (description "This package provides The teloxide's procedural macros.")
    (license license:expat)))

(define-public rust-xshell-macros-0.2
  (package
    (name "rust-xshell-macros")
    (version "0.2.7")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "xshell-macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0irm50jxdc92r0kd6yvl5p28jsfzha59brxk7z9w3jcf7z6h1b1j"))))
    (build-system cargo-build-system)
    (home-page "https://github.com/matklad/xshell")
    (synopsis "Private implementation detail of xshell crate")
    (description
     "This package provides Private implementation detail of xshell crate.")
    (license (list license:expat license:asl2.0))))

(define-public rust-xshell-0.2
  (package
    (name "rust-xshell")
    (version "0.2.7")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "xshell" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0g9pd9bfp0f35rzichic55k7p1mn8mqp607y5rimhiq14g390wly"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-xshell-macros" ,rust-xshell-macros-0.2))
       #:cargo-development-inputs (("rust-anyhow" ,rust-anyhow-1))))
    (home-page "https://github.com/matklad/xshell")
    (synopsis "Utilities for quick shell scripting in Rust")
    (description
     "This package provides Utilities for quick shell scripting in Rust.")
    (license (list license:expat license:asl2.0))))

(define-public rust-indent-write-2
  (package
    (name "rust-indent-write")
    (version "2.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "indent_write" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1hqjp80argdskrhd66g9sh542yxy8qi77j6rc69qd0l7l52rdzhc"))))
    (build-system cargo-build-system)
    (home-page "https://github.com/Lucretiel/indent-write")
    (synopsis "Simple Write adapters to add line indentation")
    (description
     "This package provides Simple Write adapters to add line indentation.")
    (license license:mpl2.0)))

(define-public rust-cool-asserts-2
  (package
    (name "rust-cool-asserts")
    (version "2.0.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "cool_asserts" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1v18dg7ifx41k2f82j3gsnpm1fg9wk5s4zv7sf42c7pnad72b7zf"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-indent-write" ,rust-indent-write-2))))
    (home-page "https://github.com/Lucretiel/cool_asserts")
    (synopsis "collection of useful testing assertions and utilities")
    (description
     "This package provides a collection of useful testing assertions and utilities.")
    (license license:mpl2.0)))

(define-public rust-vecrem-0.1
  (package
    (name "rust-vecrem")
    (version "0.1.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "vecrem" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1yj3wcy0zn5nfk6a1xvhsq6ha3qz0fw4qghn55572f4jg0l8m064"))))
    (build-system cargo-build-system)
    (home-page "https://github.com/WaffleLapkin/vecrem/")
    (synopsis
     "Cursor-like structure for fast iterative removing of elements from a vec")
    (description
     "This package provides Cursor-like structure for fast iterative removing of elements from a vec.")
    (license license:wtfpl2)))

(define-public rust-takecell-0.1
  (package
    (name "rust-takecell")
    (version "0.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "takecell" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0gpzcnazzwl6mq7xzbk9jf5zd0959h6310ls1ibapp3ccwwl7wr0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-development-inputs (("rust-once-cell" ,rust-once-cell-1))))
    (home-page "https://github.com/wafflelapkin/takecell")
    (synopsis "cell type which value can only be taken once")
    (description
     "This package provides a cell type which value can only be taken once.")
    (license license:expat)))

(define-public rust-slice-dst-1
  (package
    (name "rust-slice-dst")
    (version "1.5.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "slice-dst" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "04il428xdcfrvlixj3havp2k4x42dbd3wkk5x9y9khnplqhnf6pc"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-autocfg" ,rust-autocfg-1)
                       ("rust-erasable" ,rust-erasable-1))))
    (home-page
     "https://github.com/CAD97/pointer-utils/tree/master/crates/slice-dst")
    (synopsis "Slice-based custom DSTs")
    (description "This package provides Slice-based custom DSTs.")
    (license (list license:expat license:asl2.0))))

(define-public rust-erasable-1
  (package
    (name "rust-erasable")
    (version "1.2.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "erasable" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0z8b6k8aan6h80vp4mm327lvmrx0mdn4psyiwmj7mm41w468j4az"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-autocfg" ,rust-autocfg-1)
                       ("rust-scopeguard" ,rust-scopeguard-1))
       #:cargo-development-inputs (("rust-either" ,rust-either-1))))
    (home-page
     "https://github.com/CAD97/pointer-utils/tree/master/crates/erasable")
    (synopsis "Type-erased thin pointers")
    (description "This package provides Type-erased thin pointers.")
    (license (list license:expat license:asl2.0))))

(define-public rust-rc-box-1
  (package
    (name "rust-rc-box")
    (version "1.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "rc-box" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1n1zb14isg05apb21n8h4b0bm58kvrdc5aydq8q402dzx9chfsg0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-erasable" ,rust-erasable-1)
                       ("rust-slice-dst" ,rust-slice-dst-1)
                       ("rust-unsize" ,rust-unsize-1))))
    (home-page
     "https://github.com/CAD97/pointer-utils/tree/master/crates/rc-box")
    (synopsis "Known unique versions of Rc and Arc")
    (description "This package provides Known unique versions of Rc and Arc.")
    (license (list license:expat license:asl2.0))))

(define-public rust-teloxide-core-0.10
  (package
    (name "rust-teloxide-core")
    (version "0.10.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "teloxide-core" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0s9gyq3rcdb0y2gn8hkdckyd91prq09qv319p1iszrqhxykl45ly"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-bitflags" ,rust-bitflags-1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-derive-more" ,rust-derive-more-0.99)
                       ("rust-either" ,rust-either-1)
                       ("rust-futures" ,rust-futures-0.3)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-mime" ,rust-mime-0.3)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-pin-project" ,rust-pin-project-1)
                       ("rust-rc-box" ,rust-rc-box-1)
                       ("rust-reqwest" ,rust-reqwest-0.11)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-serde-with" ,rust-serde-with-1)
                       ("rust-take-mut" ,rust-take-mut-0.2)
                       ("rust-takecell" ,rust-takecell-0.1)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-util" ,rust-tokio-util-0.7)
                       ("rust-url" ,rust-url-2)
                       ("rust-uuid" ,rust-uuid-1)
                       ("rust-vecrem" ,rust-vecrem-0.1))
       #:cargo-development-inputs (("rust-aho-corasick" ,rust-aho-corasick-0.7)
                                   ("rust-cool-asserts" ,rust-cool-asserts-2)
                                   ("rust-indexmap" ,rust-indexmap-1)
                                   ("rust-itertools" ,rust-itertools-0.10)
                                   ("rust-pretty-env-logger" ,rust-pretty-env-logger-0.4)
                                   ("rust-ron" ,rust-ron-0.7)
                                   ("rust-tokio" ,rust-tokio-1)
                                   ("rust-xshell" ,rust-xshell-0.2))))
    (home-page "https://github.com/teloxide/teloxide")
    (synopsis "Core part of the `teloxide` library - telegram bot API client")
    (description
     "This package provides Core part of the `teloxide` library - telegram bot API client.")
    (license license:expat)))

(define-public rust-dptree-0.3
  (package
    (name "rust-dptree")
    (version "0.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "dptree" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0p165vl0gc91nc5h00n340bp5qa4qb92xpvn0l7c6ygcnpd7a4fq"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-futures" ,rust-futures-0.3))
       #:cargo-development-inputs (("rust-maplit" ,rust-maplit-1)
                                   ("rust-tokio" ,rust-tokio-1))))
    (home-page "https://github.com/p0lunin/dptree")
    (synopsis "An asynchronous event dispatch mechanism for Rust")
    (description
     "This package provides An asynchronous event dispatch mechanism for Rust.")
    (license license:expat)))

(define-public rust-tempfile-3
  (package
    (name "rust-tempfile")
    (version "3.6.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tempfile" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1mm6n3ijfsnk7grbbws3fc9qy4y5n3pshixa19wmhzimfqj47h1i"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-autocfg" ,rust-autocfg-1)
                       ("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-fastrand" ,rust-fastrand-1)
                       ("rust-redox-syscall" ,rust-redox-syscall-0.3)
                       ("rust-rustix" ,rust-rustix-0.37)
                       ("rust-windows-sys" ,rust-windows-sys-0.48))
       #:cargo-development-inputs (("rust-doc-comment" ,rust-doc-comment-0.3))))
    (home-page "https://stebalien.com/projects/tempfile-rs/")
    (synopsis "library for managing temporary files and directories.")
    (description
     "This package provides a library for managing temporary files and directories.")
    (license (list license:expat license:asl2.0))))

(define-public rust-tokio-retry-0.3
  (package
    (name "rust-tokio-retry")
    (version "0.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tokio-retry" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0kr1hnm5dmb9gfkby88yg2xj8g6x4i4gipva0c8ca3xyxhvfnmvz"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-pin-project" ,rust-pin-project-1)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-tokio" ,rust-tokio-1))
       #:cargo-development-inputs (("rust-tokio" ,rust-tokio-1))))
    (home-page "https://github.com/srijs/rust-tokio-retry")
    (synopsis "Extensible, asynchronous retry behaviours for futures/tokio")
    (description
     "This package provides Extensible, asynchronous retry behaviours for futures/tokio.")
    (license license:expat)))

(define-public rust-futures-rustls-0.24
  (package
    (name "rust-futures-rustls")
    (version "0.24.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "futures-rustls" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0a1acak02s42wh6qjmjyviscc5j77qsh1qrqd023hdqqikv3rg9m"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-futures-io" ,rust-futures-io-0.3)
                       ("rust-rustls" ,rust-rustls-0.21))
       #:cargo-development-inputs (("rust-futures-util" ,rust-futures-util-0.3)
                                   ("rust-lazy-static" ,rust-lazy-static-1)
                                   ("rust-rustls-pemfile" ,rust-rustls-pemfile-1)
                                   ("rust-rustls-webpki" ,rust-rustls-webpki-0.100)
                                   ("rust-smol" ,rust-smol-1)
                                   ("rust-webpki-roots" ,rust-webpki-roots-0.23))))
    (home-page "https://github.com/quininer/futures-rustls")
    (synopsis "Asynchronous TLS/SSL streams for futures using Rustls")
    (description
     "This package provides Asynchronous TLS/SSL streams for futures using Rustls.")
    (license (list license:expat license:asl2.0))))

(define-public rust-crc16-0.4
  (package
    (name "rust-crc16")
    (version "0.4.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "crc16" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1zzwb5iv51wnh96532cxkk4aa8ys47rhzrjy98wqcys25ks8k01k"))))
    (build-system cargo-build-system)
    (home-page "https://github.com/blackbeam/rust-crc16")
    (synopsis "CRC16 implementation")
    (description "This package provides a CRC16 implementation.")
    (license license:expat)))

(define-public rust-async-native-tls-0.4
  (package
    (name "rust-async-native-tls")
    (version "0.4.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "async-native-tls" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1zhkka5azpr03wg2bswabmwcwcqbdia17h2d17hk4wk47kn4qzfm"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-native-tls" ,rust-native-tls-0.2)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-url" ,rust-url-2))
       #:cargo-development-inputs (("rust-async-std" ,rust-async-std-1)
                                   ("rust-cfg-if" ,rust-cfg-if-0.1)
                                   ("rust-env-logger" ,rust-env-logger-0.7)
                                   ("rust-futures" ,rust-futures-0.3)
                                   ("rust-tokio" ,rust-tokio-1))))
    (home-page "https://docs.rs/crate/async-native-tls/")
    (synopsis "Native TLS using futures")
    (description "This package provides Native TLS using futures.")
    (license (list license:expat license:asl2.0))))

(define-public rust-redis-0.24
  (package
    (name "rust-redis")
    (version "0.24.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "redis" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1kc4mzvc6fmlh25l0j30hkk5gjb2daprvkv7ing4f6qxpv5xk065"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-ahash" ,rust-ahash-0.8)
                       ("rust-arc-swap" ,rust-arc-swap-1)
                       ("rust-async-native-tls" ,rust-async-native-tls-0.4)
                       ("rust-async-std" ,rust-async-std-1)
                       ("rust-async-trait" ,rust-async-trait-0.1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-combine" ,rust-combine-4)
                       ("rust-crc16" ,rust-crc16-0.4)
                       ("rust-futures" ,rust-futures-0.3)
                       ("rust-futures-rustls" ,rust-futures-rustls-0.24)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-itoa" ,rust-itoa-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-native-tls" ,rust-native-tls-0.2)
                       ("rust-percent-encoding" ,rust-percent-encoding-2)
                       ("rust-pin-project-lite" ,rust-pin-project-lite-0.2)
                       ("rust-r2d2" ,rust-r2d2-0.8)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-rustls" ,rust-rustls-0.21)
                       ("rust-rustls-native-certs" ,rust-rustls-native-certs-0.6)
                       ("rust-rustls-pemfile" ,rust-rustls-pemfile-1)
                       ("rust-rustls-webpki" ,rust-rustls-webpki-0.101)
                       ("rust-ryu" ,rust-ryu-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-sha1-smol" ,rust-sha1-smol-1)
                       ("rust-socket2" ,rust-socket2-0.4)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-native-tls" ,rust-tokio-native-tls-0.3)
                       ("rust-tokio-retry" ,rust-tokio-retry-0.3)
                       ("rust-tokio-rustls" ,rust-tokio-rustls-0.24)
                       ("rust-tokio-util" ,rust-tokio-util-0.7)
                       ("rust-url" ,rust-url-2)
                       ("rust-webpki-roots" ,rust-webpki-roots-0.23))
       #:cargo-development-inputs (("rust-anyhow" ,rust-anyhow-1)
                                   ("rust-assert-approx-eq" ,rust-assert-approx-eq-1)
                                   ("rust-criterion" ,rust-criterion-0.4)
                                   ("rust-fnv" ,rust-fnv-1)
                                   ("rust-futures" ,rust-futures-0.3)
                                   ("rust-once-cell" ,rust-once-cell-1)
                                   ("rust-partial-io" ,rust-partial-io-0.5)
                                   ("rust-quickcheck" ,rust-quickcheck-1)
                                   ("rust-rand" ,rust-rand-0.8)
                                   ("rust-socket2" ,rust-socket2-0.4)
                                   ("rust-tempfile" ,rust-tempfile-3)
                                   ("rust-tokio" ,rust-tokio-1))))
    (home-page "https://github.com/redis-rs/redis-rs")
    (synopsis "Redis driver for Rust")
    (description "This package provides Redis driver for Rust.")
    (license license:bsd-3)))

(define-public rust-deadpool-redis-0.14
  (package
    (name "rust-deadpool-redis")
    (version "0.14.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "deadpool-redis" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1zqzd36bl9jpq07k907rbc408h5w6dmli7ylyshhcgcr1qdkiwin"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-deadpool" ,rust-deadpool-0.10)
                       ("rust-redis" ,rust-redis-0.24)
                       ("rust-serde" ,rust-serde-1))
       #:cargo-development-inputs (("rust-config" ,rust-config-0.13)
                                   ("rust-dotenv" ,rust-dotenv-0.15)
                                   ("rust-futures" ,rust-futures-0.3)
                                   ("rust-redis" ,rust-redis-0.24)
                                   ("rust-tokio" ,rust-tokio-1))))
    (home-page "https://github.com/bikeshedder/deadpool")
    (synopsis "Dead simple async pool for redis")
    (description "This package provides Dead simple async pool for redis.")
    (license (list license:expat license:asl2.0))))

(define-public rust-aquamarine-0.5
  (package
    (name "rust-aquamarine")
    (version "0.5.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "aquamarine" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0kizryj8h9zgwrb0q5q5f1c6bg56gnbg19wan5g06icj6141bk11"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-include-dir" ,rust-include-dir-0.7)
                       ("rust-itertools" ,rust-itertools-0.10)
                       ("rust-proc-macro-error" ,rust-proc-macro-error-1)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))
       #:cargo-development-inputs (("rust-pretty-assertions" ,rust-pretty-assertions-1))))
    (home-page "https://github.com/mersinvald/aquamarine")
    (synopsis "mermaid.js integration for rustdoc")
    (description "This package provides a mermaid.js integration for rustdoc.")
    (license license:expat)))

(define-public rust-teloxide-0.13
  (package
    (name "rust-teloxide")
    (version "0.13.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "teloxide" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "05mwrh4c58l9mlxv6xmqxwq5365rhdzpryh33i2r06xj7qldsyaz"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-aquamarine" ,rust-aquamarine-0.5)
                       ("rust-axum" ,rust-axum-0.7)
                       ("rust-bincode" ,rust-bincode-1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-deadpool-redis" ,rust-deadpool-redis-0.14)
                       ("rust-derive-more" ,rust-derive-more-0.99)
                       ("rust-dptree" ,rust-dptree-0.3)
                       ("rust-either" ,rust-either-1)
                       ("rust-futures" ,rust-futures-0.3)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-mime" ,rust-mime-0.3)
                       ("rust-pin-project" ,rust-pin-project-1)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-cbor" ,rust-serde-cbor-0.11)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-sqlx" ,rust-sqlx-0.7)
                       ("rust-teloxide-core" ,rust-teloxide-core-0.10)
                       ("rust-teloxide-macros" ,rust-teloxide-macros-0.8)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-stream" ,rust-tokio-stream-0.1)
                       ("rust-tokio-util" ,rust-tokio-util-0.7)
                       ("rust-tower" ,rust-tower-0.4)
                       ("rust-tower-http" ,rust-tower-http-0.5)
                       ("rust-url" ,rust-url-2))
       #:cargo-development-inputs (("rust-chrono" ,rust-chrono-0.4)
                                   ("rust-pretty-env-logger" ,rust-pretty-env-logger-0.5)
                                   ("rust-rand" ,rust-rand-0.8)
                                   ("rust-reqwest" ,rust-reqwest-0.11)
                                   ("rust-serde" ,rust-serde-1)
                                   ("rust-serde-json" ,rust-serde-json-1)
                                   ("rust-tokio" ,rust-tokio-1)
                                   ("rust-tokio-stream" ,rust-tokio-stream-0.1))))
    (home-page "https://github.com/teloxide/teloxide")
    (synopsis "An elegant Telegram bots framework for Rust")
    (description
     "This package provides An elegant Telegram bots framework for Rust.")
    (license license:expat)))

(define-public rust-smtp-proto-0.1
  (package
    (name "rust-smtp-proto")
    (version "0.1.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "smtp-proto" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0fm5cc87hn52cjyjbkkv9xm9v4d90madcamhpbgd9w47s4ysvf2i"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/stalwartlabs/smtp-proto")
    (synopsis "SMTP protocol parser")
    (description "This package provides SMTP protocol parser.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-mail-parser-0.9
  (package
    (name "rust-mail-parser")
    (version "0.9.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mail-parser" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1z7r9b4fn852s3kqi2mzlg01isfn6wxw9frh6dbsyzxiv3jvkhwk"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-encoding-rs" ,rust-encoding-rs-0.8)
                       ("rust-serde" ,rust-serde-1))
       #:cargo-development-inputs (("rust-bincode" ,rust-bincode-1)
                                   ("rust-chrono" ,rust-chrono-0.4)
                                   ("rust-serde" ,rust-serde-1)
                                   ("rust-serde-json" ,rust-serde-json-1))))
    (home-page "https://github.com/stalwartlabs/mail-parser")
    (synopsis "Fast and robust e-mail parsing library for Rust")
    (description
     "This package provides Fast and robust e-mail parsing library for Rust.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-mail-builder-0.3
  (package
    (name "rust-mail-builder")
    (version "0.3.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mail-builder" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1jzg9y92xbdj2glkpbakhrgv0scd1ih9a2vmxvr81vbha8fqgx95"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-gethostname" ,rust-gethostname-0.4))
       #:cargo-development-inputs (("rust-mail-parser" ,rust-mail-parser-0.9)
                                   ("rust-serde" ,rust-serde-1)
                                   ("rust-serde-json" ,rust-serde-json-1)
                                   ("rust-serde-yaml" ,rust-serde-yaml-0.9))))
    (home-page "https://github.com/stalwartlabs/mail-builder")
    (synopsis "E-mail builder library for Rust")
    (description "This package provides E-mail builder library for Rust.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-mail-auth-0.4
  (package
    (name "rust-mail-auth")
    (version "0.4.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mail-auth" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1pkhyh790k6yslrc0z5ry9jjbb2wky6anghc6sndb8v6vrbxdncv"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-ahash" ,rust-ahash-0.8)
                       ("rust-ed25519-dalek" ,rust-ed25519-dalek-2)
                       ("rust-flate2" ,rust-flate2-1)
                       ("rust-hickory-resolver" ,rust-hickory-resolver-0.24)
                       ("rust-lru-cache" ,rust-lru-cache-0.1)
                       ("rust-mail-builder" ,rust-mail-builder-0.3)
                       ("rust-mail-parser" ,rust-mail-parser-0.9)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-quick-xml" ,rust-quick-xml-0.32)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-ring" ,rust-ring-0.17)
                       ("rust-rsa" ,rust-rsa-0.9)
                       ("rust-rustls-pemfile" ,rust-rustls-pemfile-2)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-sha1" ,rust-sha1-0.10)
                       ("rust-sha2" ,rust-sha2-0.10)
                       ("rust-zip" ,rust-zip-2))
       #:cargo-development-inputs (("rust-rustls-pemfile" ,rust-rustls-pemfile-2)
                                   ("rust-tokio" ,rust-tokio-1))))
    (home-page "https://github.com/stalwartlabs/mail-auth")
    (synopsis "DKIM, ARC, SPF and DMARC library for Rust")
    (description
     "This package provides DKIM, ARC, SPF and DMARC library for Rust.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-mail-send-0.4
  (package
    (name "rust-mail-send")
    (version "0.4.9")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mail-send" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0m35h72izqs5gga8axqqqs8yjira4hlkniqbg7jniv80rwjmsmvs"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-base64" ,rust-base64-0.22)
                       ("rust-gethostname" ,rust-gethostname-0.4)
                       ("rust-mail-auth" ,rust-mail-auth-0.4)
                       ("rust-mail-builder" ,rust-mail-builder-0.3)
                       ("rust-mail-parser" ,rust-mail-parser-0.9)
                       ("rust-md5" ,rust-md5-0.7)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-rustls" ,rust-rustls-0.23)
                       ("rust-rustls-pki-types" ,rust-rustls-pki-types-1)
                       ("rust-smtp-proto" ,rust-smtp-proto-0.1)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-rustls" ,rust-tokio-rustls-0.26)
                       ("rust-webpki-roots" ,rust-webpki-roots-0.26))
       #:cargo-development-inputs (("rust-env-logger" ,rust-env-logger-0.11)
                                   ("rust-tokio" ,rust-tokio-1))))
    (home-page "https://github.com/stalwartlabs/mail-send")
    (synopsis "E-mail delivery library with SMTP and DKIM support")
    (description
     "This package provides E-mail delivery library with SMTP and DKIM support.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-confy-0.6
  (package
    (name "rust-confy")
    (version "0.6.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "confy" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "18795hfza7g0jwbvldl5rammmk3jdaxq5b6w9b1pvw3h130g9ca5"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-directories" ,rust-directories-5)
                       ("rust-ron" ,rust-ron-0.8)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-yaml" ,rust-serde-yaml-0.9)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-toml" ,rust-toml-0.8))
       #:cargo-development-inputs (("rust-serde-derive" ,rust-serde-derive-1)
                                   ("rust-tempfile" ,rust-tempfile-3))))
    (home-page "https://github.com/rust-cli/confy")
    (synopsis "Boilerplate-free configuration management")
    (description
     "This package provides Boilerplate-free configuration management.")
    (license (list license:expat license:x11 license:asl2.0))))

(define vcs-file?
  ;; Return true if the given file is under version control.
  (or (git-predicate (current-source-directory))
      (const #t)))                                ;not in a Git checkout

(define-public rust-kmail-alias-bot-0.1
  (package
    (name "rust-kmail-alias-bot")
    (version "0.1.1")
    (source
     (local-file "." "rkab-checkout"
                 #:recursive? #t
                 #:select? vcs-file?))
    (build-system cargo-build-system)
    ; TODO: should any of them be "native-inputs"?
    (inputs (list pkg-config openssl zstd (list zstd "lib") nss-certs))
    (arguments
     `(#:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-confy" ,rust-confy-0.6)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-mail-send" ,rust-mail-send-0.4)
                       ("rust-pretty-env-logger" ,rust-pretty-env-logger-0.5)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-reqwest" ,rust-reqwest-0.12)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-teloxide" ,rust-teloxide-0.13)
                       ("rust-tokio" ,rust-tokio-1))
       #:cargo-development-inputs (("rust-mockito" ,rust-mockito-1)
                                   ("rust-teloxide-tests" ,rust-teloxide-tests-0.2))
       #:phases
       (modify-phases %standard-phases
                       (add-after 'install 'wrap-binary
                                  (lambda* (#:key outputs inputs #:allow-other-keys)
                                    (let* ((out (assoc-ref outputs "out"))
                                           (bin (string-append out "/bin/kmail-alias-bot"))
                                           (cert-dir (string-append (assoc-ref inputs "nss-certs")
                                                                    "/etc/ssl/certs")))
                                      (wrap-program bin
                                                    `("SSL_CERT_DIR" = ,(list cert-dir)))))))))
    (home-page "https://github.com/necto/kmail-alias-bot")
    (synopsis
     "Telegram bot for easy alias management on Infomaniak kMail service.")
    (description
     "This package provides a Telegram bot for easy alias management on Infomaniak
@code{kMail} service.")
    (license license:expat)))

rust-kmail-alias-bot-0.1
