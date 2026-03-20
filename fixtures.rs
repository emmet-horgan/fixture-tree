// fixture-tree auto-generated fixture accessors

pub fn path() -> &'static std::path::Path {
    std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures")
}

pub mod unit {

    pub fn path() -> &'static std::path::Path {
        std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit")
    }

    pub mod configs {

        pub fn path() -> &'static std::path::Path {
            std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs")
        }

        pub mod fail {

            pub fn path() -> &'static std::path::Path {
                std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail")
            }

            pub fn malformed_json() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/malformed_json.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/malformed_json.json"))
            }
                
            pub fn models_duplicate_paths() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/models_duplicate_paths.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/models_duplicate_paths.json"))
            }
                
            pub fn duplicate_subsystem_key() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/duplicate_subsystem_key.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/duplicate_subsystem_key.json"))
            }
                
            pub fn duplicate_soc_key() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/duplicate_soc_key.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/duplicate_soc_key.json"))
            }
                
            pub fn os_windows() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/os_windows.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/os_windows.json"))
            }
                
            pub fn language_unknown() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/language_unknown.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/language_unknown.json"))
            }
                
            pub fn unknown_subsystem() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/unknown_subsystem.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/unknown_subsystem.json"))
            }
                
            pub mod neuroweave {

                pub fn path() -> &'static std::path::Path {
                    std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/neuroweave")
                }

                pub fn mixed_arch_hifi5_cref() -> (&'static std::path::Path, &'static str) {
                    (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/neuroweave/mixed_arch_hifi5_cref.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/neuroweave/mixed_arch_hifi5_cref.json"))
                }
                
                pub fn too_many_models() -> (&'static std::path::Path, &'static str) {
                    (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/neuroweave/too_many_models.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/neuroweave/too_many_models.json"))
                }
                
                pub fn mixed_arch_three_archs() -> (&'static std::path::Path, &'static str) {
                    (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/neuroweave/mixed_arch_three_archs.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/neuroweave/mixed_arch_three_archs.json"))
                }
                
                pub fn unknown_target() -> (&'static std::path::Path, &'static str) {
                    (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/neuroweave/unknown_target.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/neuroweave/unknown_target.json"))
                }
                
                pub fn mixed_arch() -> (&'static std::path::Path, &'static str) {
                    (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/neuroweave/mixed_arch.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/neuroweave/mixed_arch.json"))
                }
                
                pub fn subsystem_css_with_dss_nss() -> (&'static std::path::Path, &'static str) {
                    (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/neuroweave/subsystem_css_with_dss_nss.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/fail/neuroweave/subsystem_css_with_dss_nss.json"))
                }
                
            }

        }

        pub mod pass {

            pub fn path() -> &'static std::path::Path {
                std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass")
            }

            pub fn test_config_dual_subsystem() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_dual_subsystem.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_dual_subsystem.json"))
            }
                
            pub fn test_config_nss_4models() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_nss_4models.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_nss_4models.json"))
            }
                
            pub fn test_config_dss() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_dss.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_dss.json"))
            }
                
            pub fn test_config_nss_2models() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_nss_2models.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_nss_2models.json"))
            }
                
            pub fn test_config_nss() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_nss.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_nss.json"))
            }
                
            pub fn test_config_dss_3models() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_dss_3models.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_dss_3models.json"))
            }
                
            pub fn test_config_dss_4models() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_dss_4models.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_dss_4models.json"))
            }
                
            pub fn test_config_simple() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_simple.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_simple.json"))
            }
                
            pub fn test_config_dss_2models() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_dss_2models.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_dss_2models.json"))
            }
                
            pub fn test_config_nss_3models() -> (&'static std::path::Path, &'static str) {
                (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_nss_3models.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/test_config_nss_3models.json"))
            }
                
            pub mod neuroweave {

                pub fn path() -> &'static std::path::Path {
                    std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/neuroweave")
                }

                pub fn mixed_arch_nne_hifi5() -> (&'static std::path::Path, &'static str) {
                    (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/neuroweave/mixed_arch_nne_hifi5.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/unit/configs/pass/neuroweave/mixed_arch_nne_hifi5.json"))
                }
                
            }

        }

    }

}

pub mod validation {

    pub fn path() -> &'static std::path::Path {
        std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation")
    }

    pub mod configs {

        pub fn path() -> &'static std::path::Path {
            std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs")
        }

        pub mod pass {

            pub fn path() -> &'static std::path::Path {
                std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass")
            }

            pub mod neuroweave {

                pub fn path() -> &'static std::path::Path {
                    std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave")
                }

                pub mod ad71270 {

                    pub fn path() -> &'static std::path::Path {
                        std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270")
                    }

                    pub mod xtsc {

                        pub fn path() -> &'static std::path::Path {
                            std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc")
                        }

                        pub fn dss_multi_model_hifi5() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/dss_multi_model_hifi5.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/dss_multi_model_hifi5.json"))
                        }
                
                        pub fn nss_hifi5_softmax_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_softmax_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_softmax_16x8.json"))
                        }
                
                        pub fn nss_hifi5_softmax_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_softmax_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_softmax_16x4.json"))
                        }
                
                        pub fn nss_hifi5_softmax_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_softmax_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_softmax_8x4.json"))
                        }
                
                        pub fn nss_hifi5_gru_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_gru_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_gru_8x8.json"))
                        }
                
                        pub fn nss_nne_add_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_add_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_add_8x4.json"))
                        }
                
                        pub fn nss_nne_conv2d_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_conv2d_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_conv2d_16x4.json"))
                        }
                
                        pub fn nss_hifi5_gru_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_gru_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_gru_16x4.json"))
                        }
                
                        pub fn nss_nne_strided_slicing_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_strided_slicing_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_strided_slicing_16x4.json"))
                        }
                
                        pub fn nss_nne_softmax_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_softmax_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_softmax_8x4.json"))
                        }
                
                        pub fn zephyr_nss_hifi5_add_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/zephyr_nss_hifi5_add_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/zephyr_nss_hifi5_add_8x4.json"))
                        }
                
                        pub fn nss_hifi5_timewise_conv2d_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_timewise_conv2d_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_timewise_conv2d_8x4.json"))
                        }
                
                        pub fn nss_nne_attention_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_attention_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_attention_8x4.json"))
                        }
                
                        pub fn nss_nne_lstm_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_lstm_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_lstm_16x8.json"))
                        }
                
                        pub fn nss_nne_transpose_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_transpose_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_transpose_8x8.json"))
                        }
                
                        pub fn nss_nne_fully_connected_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_fully_connected_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_fully_connected_16x8.json"))
                        }
                
                        pub fn nss_hifi5_transpose_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_transpose_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_transpose_8x4.json"))
                        }
                
                        pub fn nss_nne_conv2d_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_conv2d_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_conv2d_8x8.json"))
                        }
                
                        pub fn nss_hifi5_fully_connected_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_fully_connected_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_fully_connected_16x8.json"))
                        }
                
                        pub fn nss_hifi5_gru_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_gru_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_gru_16x8.json"))
                        }
                
                        pub fn nss_nne_conv2d_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_conv2d_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_conv2d_16x8.json"))
                        }
                
                        pub fn nss_nne_softmax_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_softmax_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_softmax_8x8.json"))
                        }
                
                        pub fn nss_nne_gru_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_gru_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_gru_8x4.json"))
                        }
                
                        pub fn nss_nne_attention_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_attention_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_attention_16x8.json"))
                        }
                
                        pub fn nss_hifi5_strided_slicing_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_strided_slicing_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_strided_slicing_16x8.json"))
                        }
                
                        pub fn nss_nne_lstm_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_lstm_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_lstm_8x8.json"))
                        }
                
                        pub fn nss_nne_conv2d_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_conv2d_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_conv2d_8x4.json"))
                        }
                
                        pub fn nss_nne_transpose_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_transpose_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_transpose_8x4.json"))
                        }
                
                        pub fn nss_nne_fully_connected_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_fully_connected_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_fully_connected_8x8.json"))
                        }
                
                        pub fn nss_hifi5_timewise_conv2d_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_timewise_conv2d_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_timewise_conv2d_16x4.json"))
                        }
                
                        pub fn nss_hifi5_fully_connected_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_fully_connected_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_fully_connected_8x4.json"))
                        }
                
                        pub fn nss_nne_timewise_conv2d_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_timewise_conv2d_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_timewise_conv2d_8x8.json"))
                        }
                
                        pub fn nss_hifi5_timewise_conv2d_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_timewise_conv2d_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_timewise_conv2d_16x8.json"))
                        }
                
                        pub fn nss_hifi5_attention_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_attention_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_attention_8x4.json"))
                        }
                
                        pub fn nss_hifi5_conv2d_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_conv2d_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_conv2d_16x4.json"))
                        }
                
                        pub fn nss_hifi5_transpose_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_transpose_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_transpose_16x4.json"))
                        }
                
                        pub fn nss_hifi5_reduce_mean_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_reduce_mean_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_reduce_mean_16x4.json"))
                        }
                
                        pub fn nss_nne_timewise_conv2d_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_timewise_conv2d_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_timewise_conv2d_16x8.json"))
                        }
                
                        pub fn nss_nne_lstm_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_lstm_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_lstm_8x4.json"))
                        }
                
                        pub fn nss_hifi5_strided_slicing_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_strided_slicing_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_strided_slicing_8x4.json"))
                        }
                
                        pub fn nss_hifi5_add_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_add_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_add_16x4.json"))
                        }
                
                        pub fn nss_nne_reduce_mean_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_reduce_mean_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_reduce_mean_8x8.json"))
                        }
                
                        pub fn nss_hifi5_reduce_mean_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_reduce_mean_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_reduce_mean_8x4.json"))
                        }
                
                        pub fn dss_hifi5_add_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/dss_hifi5_add_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/dss_hifi5_add_8x4.json"))
                        }
                
                        pub fn nss_hifi5_softmax_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_softmax_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_softmax_8x8.json"))
                        }
                
                        pub fn nss_hifi5_reduce_mean_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_reduce_mean_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_reduce_mean_16x8.json"))
                        }
                
                        pub fn nss_nne_add_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_add_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_add_16x4.json"))
                        }
                
                        pub fn nss_nne_attention_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_attention_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_attention_16x4.json"))
                        }
                
                        pub fn nss_nne_gru_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_gru_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_gru_16x4.json"))
                        }
                
                        pub fn nss_hifi5_transpose_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_transpose_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_transpose_16x8.json"))
                        }
                
                        pub fn nss_nne_reduce_mean_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_reduce_mean_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_reduce_mean_8x4.json"))
                        }
                
                        pub fn nss_nne_transpose_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_transpose_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_transpose_16x4.json"))
                        }
                
                        pub fn nss_nne_person_detect_int8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_person_detect_int8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_person_detect_int8.json"))
                        }
                
                        pub fn nss_hifi5_add_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_add_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_add_8x4.json"))
                        }
                
                        pub fn nss_hifi5_conv2d_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_conv2d_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_conv2d_8x8.json"))
                        }
                
                        pub fn nss_nne_add_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_add_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_add_8x8.json"))
                        }
                
                        pub fn nss_hifi5_lstm_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_lstm_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_lstm_16x4.json"))
                        }
                
                        pub fn nss_nne_multi_model_nne() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_multi_model_nne.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_multi_model_nne.json"))
                        }
                
                        pub fn nss_hifi5_lstm_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_lstm_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_lstm_8x4.json"))
                        }
                
                        pub fn nss_nne_micro_speech_lstm() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_micro_speech_lstm.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_micro_speech_lstm.json"))
                        }
                
                        pub fn nss_nne_multi_model_hifi5() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_multi_model_hifi5.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_multi_model_hifi5.json"))
                        }
                
                        pub fn nss_hifi5_attention_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_attention_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_attention_8x8.json"))
                        }
                
                        pub fn nss_nne_softmax_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_softmax_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_softmax_16x8.json"))
                        }
                
                        pub fn nss_nne_strided_slicing_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_strided_slicing_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_strided_slicing_16x8.json"))
                        }
                
                        pub fn nss_nne_reduce_mean_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_reduce_mean_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_reduce_mean_16x8.json"))
                        }
                
                        pub fn nss_hifi5_attention_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_attention_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_attention_16x8.json"))
                        }
                
                        pub fn nss_nne_timewise_conv2d_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_timewise_conv2d_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_timewise_conv2d_8x4.json"))
                        }
                
                        pub fn nss_hifi5_reduce_mean_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_reduce_mean_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_reduce_mean_8x8.json"))
                        }
                
                        pub fn nss_hifi5_conv2d_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_conv2d_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_conv2d_16x8.json"))
                        }
                
                        pub fn dss_hifi5_softmax_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/dss_hifi5_softmax_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/dss_hifi5_softmax_8x8.json"))
                        }
                
                        pub fn nss_nne_add_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_add_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_add_16x8.json"))
                        }
                
                        pub fn nss_hifi5_lstm_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_lstm_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_lstm_16x8.json"))
                        }
                
                        pub fn nss_hifi5_attention_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_attention_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_attention_16x4.json"))
                        }
                
                        pub fn nss_hifi5_lstm_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_lstm_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_lstm_8x8.json"))
                        }
                
                        pub fn nss_hifi5_strided_slicing_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_strided_slicing_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_strided_slicing_16x4.json"))
                        }
                
                        pub fn nss_nne_lstm_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_lstm_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_lstm_16x4.json"))
                        }
                
                        pub fn nss_hifi5_micro_speech_lstm() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_micro_speech_lstm.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_micro_speech_lstm.json"))
                        }
                
                        pub fn nss_hifi5_fully_connected_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_fully_connected_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_fully_connected_16x4.json"))
                        }
                
                        pub fn nss_hifi5_add_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_add_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_add_16x8.json"))
                        }
                
                        pub fn nss_hifi5_add_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_add_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_add_8x8.json"))
                        }
                
                        pub fn nss_nne_fully_connected_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_fully_connected_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_fully_connected_16x4.json"))
                        }
                
                        pub fn nss_nne_softmax_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_softmax_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_softmax_16x4.json"))
                        }
                
                        pub fn nss_hifi5_conv2d_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_conv2d_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_conv2d_8x4.json"))
                        }
                
                        pub fn nss_hetero_add_8x4_hifi5_softmax_8x8_nne() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hetero_add_8x4_hifi5_softmax_8x8_nne.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hetero_add_8x4_hifi5_softmax_8x8_nne.json"))
                        }
                
                        pub fn nss_hifi5_timewise_conv2d_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_timewise_conv2d_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_timewise_conv2d_8x8.json"))
                        }
                
                        pub fn nss_nne_transpose_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_transpose_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_transpose_16x8.json"))
                        }
                
                        pub fn nss_nne_strided_slicing_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_strided_slicing_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_strided_slicing_8x4.json"))
                        }
                
                        pub fn nss_hifi5_strided_slicing_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_strided_slicing_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_strided_slicing_8x8.json"))
                        }
                
                        pub fn nss_hifi5_person_detect_int8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_person_detect_int8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_person_detect_int8.json"))
                        }
                
                        pub fn nss_nne_timewise_conv2d_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_timewise_conv2d_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_timewise_conv2d_16x4.json"))
                        }
                
                        pub fn nss_hifi5_gru_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_gru_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_gru_8x4.json"))
                        }
                
                        pub fn nss_nne_fully_connected_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_fully_connected_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_fully_connected_8x4.json"))
                        }
                
                        pub fn nss_hifi5_transpose_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_transpose_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_transpose_8x8.json"))
                        }
                
                        pub fn nss_hifi5_fully_connected_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_fully_connected_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hifi5_fully_connected_8x8.json"))
                        }
                
                        pub fn nss_nne_gru_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_gru_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_gru_8x8.json"))
                        }
                
                        pub fn nss_nne_gru_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_gru_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_gru_16x8.json"))
                        }
                
                        pub fn nss_nne_attention_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_attention_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_attention_8x8.json"))
                        }
                
                        pub fn nss_nne_reduce_mean_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_reduce_mean_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_reduce_mean_16x4.json"))
                        }
                
                        pub fn nss_nne_strided_slicing_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_strided_slicing_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_nne_strided_slicing_8x8.json"))
                        }
                
                        pub fn nss_hetero_add_8x8_hifi5_reduce_mean_8x8_nne() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hetero_add_8x8_hifi5_reduce_mean_8x8_nne.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/xtsc/nss_hetero_add_8x8_hifi5_reduce_mean_8x8_nne.json"))
                        }
                
                    }

                    pub mod vp {

                        pub fn path() -> &'static std::path::Path {
                            std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp")
                        }

                        pub fn nss_hifi5_softmax_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_softmax_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_softmax_16x8.json"))
                        }
                
                        pub fn nss_hifi5_softmax_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_softmax_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_softmax_16x4.json"))
                        }
                
                        pub fn nss_hifi5_softmax_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_softmax_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_softmax_8x4.json"))
                        }
                
                        pub fn nss_hifi5_gru_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_gru_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_gru_8x8.json"))
                        }
                
                        pub fn nss_nne_add_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_add_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_add_8x4.json"))
                        }
                
                        pub fn nss_nne_conv2d_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_conv2d_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_conv2d_16x4.json"))
                        }
                
                        pub fn nss_hifi5_gru_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_gru_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_gru_16x4.json"))
                        }
                
                        pub fn nss_nne_strided_slicing_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_strided_slicing_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_strided_slicing_16x4.json"))
                        }
                
                        pub fn nss_nne_softmax_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_softmax_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_softmax_8x4.json"))
                        }
                
                        pub fn zephyr_nss_hifi5_add_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/zephyr_nss_hifi5_add_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/zephyr_nss_hifi5_add_8x4.json"))
                        }
                
                        pub fn nss_hifi5_timewise_conv2d_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_timewise_conv2d_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_timewise_conv2d_8x4.json"))
                        }
                
                        pub fn nss_nne_attention_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_attention_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_attention_8x4.json"))
                        }
                
                        pub fn nss_nne_lstm_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_lstm_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_lstm_16x8.json"))
                        }
                
                        pub fn nss_nne_transpose_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_transpose_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_transpose_8x8.json"))
                        }
                
                        pub fn nss_nne_fully_connected_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_fully_connected_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_fully_connected_16x8.json"))
                        }
                
                        pub fn nss_hifi5_transpose_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_transpose_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_transpose_8x4.json"))
                        }
                
                        pub fn nss_nne_conv2d_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_conv2d_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_conv2d_8x8.json"))
                        }
                
                        pub fn nss_hifi5_fully_connected_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_fully_connected_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_fully_connected_16x8.json"))
                        }
                
                        pub fn nss_hifi5_gru_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_gru_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_gru_16x8.json"))
                        }
                
                        pub fn nss_nne_conv2d_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_conv2d_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_conv2d_16x8.json"))
                        }
                
                        pub fn nss_nne_softmax_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_softmax_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_softmax_8x8.json"))
                        }
                
                        pub fn nss_nne_gru_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_gru_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_gru_8x4.json"))
                        }
                
                        pub fn nss_nne_attention_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_attention_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_attention_16x8.json"))
                        }
                
                        pub fn nss_hifi5_strided_slicing_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_strided_slicing_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_strided_slicing_16x8.json"))
                        }
                
                        pub fn nss_nne_lstm_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_lstm_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_lstm_8x8.json"))
                        }
                
                        pub fn nss_nne_conv2d_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_conv2d_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_conv2d_8x4.json"))
                        }
                
                        pub fn nss_nne_transpose_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_transpose_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_transpose_8x4.json"))
                        }
                
                        pub fn nss_nne_fully_connected_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_fully_connected_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_fully_connected_8x8.json"))
                        }
                
                        pub fn nss_hifi5_timewise_conv2d_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_timewise_conv2d_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_timewise_conv2d_16x4.json"))
                        }
                
                        pub fn nss_hifi5_fully_connected_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_fully_connected_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_fully_connected_8x4.json"))
                        }
                
                        pub fn nss_nne_timewise_conv2d_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_timewise_conv2d_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_timewise_conv2d_8x8.json"))
                        }
                
                        pub fn nss_hifi5_timewise_conv2d_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_timewise_conv2d_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_timewise_conv2d_16x8.json"))
                        }
                
                        pub fn nss_hifi5_attention_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_attention_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_attention_8x4.json"))
                        }
                
                        pub fn nss_hifi5_conv2d_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_conv2d_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_conv2d_16x4.json"))
                        }
                
                        pub fn nss_hifi5_transpose_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_transpose_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_transpose_16x4.json"))
                        }
                
                        pub fn nss_hifi5_reduce_mean_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_reduce_mean_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_reduce_mean_16x4.json"))
                        }
                
                        pub fn nss_nne_timewise_conv2d_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_timewise_conv2d_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_timewise_conv2d_16x8.json"))
                        }
                
                        pub fn nss_nne_lstm_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_lstm_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_lstm_8x4.json"))
                        }
                
                        pub fn nss_hifi5_strided_slicing_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_strided_slicing_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_strided_slicing_8x4.json"))
                        }
                
                        pub fn nss_hifi5_add_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_add_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_add_16x4.json"))
                        }
                
                        pub fn nss_nne_reduce_mean_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_reduce_mean_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_reduce_mean_8x8.json"))
                        }
                
                        pub fn nss_hifi5_reduce_mean_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_reduce_mean_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_reduce_mean_8x4.json"))
                        }
                
                        pub fn nss_hifi5_softmax_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_softmax_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_softmax_8x8.json"))
                        }
                
                        pub fn nss_hifi5_reduce_mean_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_reduce_mean_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_reduce_mean_16x8.json"))
                        }
                
                        pub fn nss_nne_add_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_add_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_add_16x4.json"))
                        }
                
                        pub fn nss_nne_attention_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_attention_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_attention_16x4.json"))
                        }
                
                        pub fn nss_nne_gru_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_gru_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_gru_16x4.json"))
                        }
                
                        pub fn nss_hifi5_transpose_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_transpose_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_transpose_16x8.json"))
                        }
                
                        pub fn nss_nne_reduce_mean_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_reduce_mean_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_reduce_mean_8x4.json"))
                        }
                
                        pub fn nss_nne_transpose_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_transpose_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_transpose_16x4.json"))
                        }
                
                        pub fn nss_nne_person_detect_int8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_person_detect_int8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_person_detect_int8.json"))
                        }
                
                        pub fn nss_hifi5_add_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_add_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_add_8x4.json"))
                        }
                
                        pub fn nss_hifi5_conv2d_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_conv2d_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_conv2d_8x8.json"))
                        }
                
                        pub fn nss_nne_add_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_add_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_add_8x8.json"))
                        }
                
                        pub fn nss_hifi5_lstm_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_lstm_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_lstm_16x4.json"))
                        }
                
                        pub fn nss_nne_multi_model_nne() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_multi_model_nne.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_multi_model_nne.json"))
                        }
                
                        pub fn nss_hifi5_lstm_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_lstm_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_lstm_8x4.json"))
                        }
                
                        pub fn nss_nne_micro_speech_lstm() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_micro_speech_lstm.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_micro_speech_lstm.json"))
                        }
                
                        pub fn nss_nne_multi_model_hifi5() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_multi_model_hifi5.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_multi_model_hifi5.json"))
                        }
                
                        pub fn nss_hifi5_attention_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_attention_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_attention_8x8.json"))
                        }
                
                        pub fn nss_nne_softmax_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_softmax_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_softmax_16x8.json"))
                        }
                
                        pub fn nss_nne_strided_slicing_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_strided_slicing_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_strided_slicing_16x8.json"))
                        }
                
                        pub fn nss_nne_reduce_mean_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_reduce_mean_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_reduce_mean_16x8.json"))
                        }
                
                        pub fn nss_hifi5_attention_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_attention_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_attention_16x8.json"))
                        }
                
                        pub fn nss_nne_timewise_conv2d_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_timewise_conv2d_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_timewise_conv2d_8x4.json"))
                        }
                
                        pub fn nss_hifi5_reduce_mean_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_reduce_mean_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_reduce_mean_8x8.json"))
                        }
                
                        pub fn nss_hifi5_conv2d_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_conv2d_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_conv2d_16x8.json"))
                        }
                
                        pub fn nss_nne_add_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_add_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_add_16x8.json"))
                        }
                
                        pub fn nss_hifi5_lstm_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_lstm_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_lstm_16x8.json"))
                        }
                
                        pub fn nss_hifi5_attention_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_attention_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_attention_16x4.json"))
                        }
                
                        pub fn nss_hifi5_lstm_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_lstm_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_lstm_8x8.json"))
                        }
                
                        pub fn nss_hifi5_strided_slicing_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_strided_slicing_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_strided_slicing_16x4.json"))
                        }
                
                        pub fn nss_nne_lstm_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_lstm_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_lstm_16x4.json"))
                        }
                
                        pub fn nss_hifi5_micro_speech_lstm() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_micro_speech_lstm.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_micro_speech_lstm.json"))
                        }
                
                        pub fn nss_hifi5_fully_connected_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_fully_connected_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_fully_connected_16x4.json"))
                        }
                
                        pub fn nss_hifi5_add_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_add_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_add_16x8.json"))
                        }
                
                        pub fn nss_hifi5_add_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_add_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_add_8x8.json"))
                        }
                
                        pub fn nss_nne_fully_connected_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_fully_connected_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_fully_connected_16x4.json"))
                        }
                
                        pub fn nss_nne_softmax_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_softmax_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_softmax_16x4.json"))
                        }
                
                        pub fn nss_hifi5_conv2d_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_conv2d_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_conv2d_8x4.json"))
                        }
                
                        pub fn nss_hifi5_timewise_conv2d_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_timewise_conv2d_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_timewise_conv2d_8x8.json"))
                        }
                
                        pub fn nss_nne_transpose_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_transpose_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_transpose_16x8.json"))
                        }
                
                        pub fn nss_nne_strided_slicing_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_strided_slicing_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_strided_slicing_8x4.json"))
                        }
                
                        pub fn nss_hifi5_strided_slicing_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_strided_slicing_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_strided_slicing_8x8.json"))
                        }
                
                        pub fn nss_hifi5_person_detect_int8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_person_detect_int8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_person_detect_int8.json"))
                        }
                
                        pub fn nss_nne_timewise_conv2d_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_timewise_conv2d_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_timewise_conv2d_16x4.json"))
                        }
                
                        pub fn nss_hifi5_gru_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_gru_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_gru_8x4.json"))
                        }
                
                        pub fn nss_nne_fully_connected_8x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_fully_connected_8x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_fully_connected_8x4.json"))
                        }
                
                        pub fn nss_hifi5_transpose_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_transpose_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_transpose_8x8.json"))
                        }
                
                        pub fn nss_hifi5_fully_connected_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_fully_connected_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_hifi5_fully_connected_8x8.json"))
                        }
                
                        pub fn nss_nne_gru_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_gru_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_gru_8x8.json"))
                        }
                
                        pub fn nss_nne_gru_16x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_gru_16x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_gru_16x8.json"))
                        }
                
                        pub fn nss_nne_attention_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_attention_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_attention_8x8.json"))
                        }
                
                        pub fn nss_nne_reduce_mean_16x4() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_reduce_mean_16x4.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_reduce_mean_16x4.json"))
                        }
                
                        pub fn nss_nne_strided_slicing_8x8() -> (&'static std::path::Path, &'static str) {
                            (std::path::Path::new("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_strided_slicing_8x8.json"), include_str!("//home/ehorgan/weaver/crates/weaver-test-utils/fixtures/validation/configs/pass/neuroweave/ad71270/vp/nss_nne_strided_slicing_8x8.json"))
                        }
                
                    }

                }

            }

        }

    }

}

