import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Cybertechnica",
  description: "A guide for understanding Confidential Computing and TEEs",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/intro' }
    ],

    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Intro', link: '/intro' },
          { text: 'General concepts', collapsed: true, items: [
            { text: 'Security features', link: '/general/1_security_features'},
            { text: 'Attestation', link: '/general/2_attestation'},
            { text: 'Sealing', link: '/general/3_sealing'},
            { text: 'Key generation and management', link: '/general/4_keygen_protections'},
            { text: 'Use-cases and applications', link: '/general/5_use_cases'},
            { text: 'Privacy and security concerns', link: '/general/6_not_perfect'}
            ] 
          },
          {
            text: 'TPMs', collapsed: true, items: [
              { text: 'General overview', link: '/tpm/1_intro'},
              { text: 'Try it, you might already have it', link: '/tpm/2_try_it'},
              { text: 'Examples and use-cases', link: '/tpm/3_examples'},
              { text: 'TPM functions and features', link: '/tpm/4_tpm_features'},
              { text: 'A tale of keys', link: '/tpm/5_tale_of_keys'},
              { text: 'Remote attestation implementation', link: '/tpm/6_RA'},
              { text: 'Sealing and disk encryption', link: '/tpm/7_Sealing'}
            ]
          },
          {
            text: 'AMD SEV SNP', collapsed: true, items: [
              { text: 'Under construction', link: '/amd_sev_snp/intro'},
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

    socialLinks: [
      { icon: 'github', link: 'https://github.com/cybertechnica/confidential-computing-guide' }
    ]
  }
})
