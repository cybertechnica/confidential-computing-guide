import { defineConfig } from 'vitepress'
import { withMermaid } from "vitepress-plugin-mermaid";

// https://vitepress.dev/reference/site-config
export default withMermaid(
  defineConfig({
    title: "Cybertechnica.io",
    description: "A guide for understanding Confidential Computing and TEEs",
    base: "/",
    themeConfig: {
      // https://vitepress.dev/reference/default-theme-config
      search: {
        provider: 'local'
      }, 
      nav: [
        { text: 'Home', link: '/' },
        { text: 'Guide', link: '/intro' }
      ],

      sidebar: [
        {
          text: 'Guide',
          items: [
            { text: 'Intro', link: '/intro' },
            { text: 'Comparing the different technologies', link: '/general/comparison'},
            { text: 'General concepts', collapsed: true, items: [
              { text: 'Security features', link: '/general/1_security_features'},
              { text: 'Attestation', link: '/general/2_attestation'},
              { text: 'Sealing', link: '/general/3_sealing'},
              { text: 'Key generation and management', link: '/general/4_keygen_protections'},
              { text: 'Use-cases and applications', link: '/general/5_use_cases'},
              { text: 'Privacy and security concerns', link: '/general/6_not_perfect'},
              
              ] 
            },
            {
              text: 'TPMs', collapsed: true, items: [
                { text: 'General overview', link: '/tpm/1_intro'},
                { text: 'Try it, you might already have it', link: '/tpm/2_try_it'},
                { text: 'Concepts', link: '/tpm/3_concepts'},
                { text: 'TPM features', link: '/tpm/4_tpm_features'},
                { text: 'A tale of keys', link: '/tpm/5_tale_of_keys'},
                { text: 'Remote attestation implementation', link: '/tpm/6_RA'},
                { text: 'Sealing and disk encryption', link: '/tpm/7_Sealing'},
                { text: 'Advanced TPM', link: '/tpm/advanced_tpm'}
              ]
            },
            {
              text: 'AMD SEV-SNP', collapsed: true, items: [
                { text: 'Quick intro', link: '/amd_sev_snp/intro'},
                { text: 'Features', link: '/amd_sev_snp/features'},
                { text: 'Installation and setup', link: '/amd_sev_snp/installation'},
                { text: 'Remote Attestation implementation', link: '/amd_sev_snp/remote_attestation'},
                { text: 'Practical example', link: '/amd_sev_snp/practical_example'},
              ]
            },
            {
              text: 'Nvidia H100', collapsed: true, items: [
                { text: 'Under construction', link: '/nvidia_h100/intro'},
              ]
            },
            {
              text: 'SGX', collapsed: true, items: [
                { text: 'Under construction', link: '/sgx/intro'},
              ]
            },          
            {
              text: 'TDX', collapsed: true, items: [
                { text: 'Under construction', link: '/tdx/intro'},
              ]
            },
            {
              text: 'ARM CCA', collapsed: true, items: [
                { text: 'Under construction', link: '/arm_cca/intro'},
              ]
            }
          ]
        }
      ],
      footer: {
        message: 'Released under Apache 2.0 License.',
        copyright: 'Copyright © 2025 Cybertechnica.io'
      },
      socialLinks: [
        { icon: 'github', link: 'https://github.com/cybertechnica/confidential-computing-guide' }
      ]
    },
    mermaid: {

    },
    mermaidPlugin: {
      class: "mermaid", // set additional css classes for parent container 
    },
  })

)
  