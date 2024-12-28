import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Cybertechnica",
  description: "A guide for understanding Confidential Computing and TEEs",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/confidential-computing-guide' },
      { text: 'Guide', link: '/confidential-computing-guide/intro' }
    ],

    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Intro', link: '/confidential-computing-guide/intro' },
          { text: 'General concepts', collapsed: true, items: [
            { text: 'Security features', link: '/confidential-computing-guide/general/1_security_features'},
            { text: 'Attestation', link: '/confidential-computing-guide/general/2_attestation'},
            { text: 'Sealing', link: '/confidential-computing-guide/general/3_sealing'},
            { text: 'Key generation and management', link: '/confidential-computing-guide/general/4_keygen_protections'},
            { text: 'Use-cases and applications', link: '/confidential-computing-guide/general/5_use_cases'},
            { text: 'Privacy and security concerns', link: '/confidential-computing-guide/general/6_not_perfect'}
            ] 
          },
          {
            text: 'TPMs', collapsed: true, items: [
              { text: 'General overview', link: '/confidential-computing-guide/tpm/1_intro'},
              { text: 'Try it, you might already have it', link: '/confidential-computing-guide/tpm/2_try_it'},
              { text: 'Examples and use-cases', link: '/confidential-computing-guide/tpm/3_examples'},
              { text: 'TPM functions and features', link: '/confidential-computing-guide/tpm/4_tpm_features'},
              { text: 'A tale of keys', link: '/confidential-computing-guide/tpm/5_tale_of_keys'},
              { text: 'Remote attestation implementation', link: '/confidential-computing-guide/tpm/6_RA'},
              { text: 'Sealing and disk encryption', link: '/confidential-computing-guide/tpm/7_Sealing'}
            ]
          },
          {
            text: 'AMD SEV SNP', collapsed: true, items: [
              { text: 'Under construction', link: '/confidential-computing-guide/amd_sev_snp/intro'},
            ]
          },
          {
            text: 'Nvidia H100', collapsed: true, items: [
              { text: 'Under construction', link: '/confidential-computing-guide/nvidia_h100/intro'},
            ]
          },
          {
            text: 'SGX', collapsed: true, items: [
              { text: 'Under construction', link: '/confidential-computing-guide/sgx/intro'},
            ]
          },          
          {
            text: 'TDX', collapsed: true, items: [
              { text: 'Under construction', link: '/confidential-computing-guide/tdx/intro'},
            ]
          },
          {
            text: 'ARM CCA', collapsed: true, items: [
              { text: 'Under construction', link: '/confidential-computing-guide/arm_cca/intro'},
            ]
          }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/cybertechnica/confidential-computing-guide' }
    ]
  }
})
