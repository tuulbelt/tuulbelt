import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Tuulbelt',
  description: 'Zero-dependency tools and utilities for modern software development',
  base: '/tuulbelt/',
  srcExclude: ['**/setup/**', '**/claude-code-*.md', '**/tool-template.md'],

  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Tools', link: '/tools/' },
      { text: 'Guide', link: '/guide/philosophy' },
      { text: 'GitHub', link: 'https://github.com/tuulbelt/tuulbelt' }
    ],

    sidebar: {
      '/tools/': [
        {
          text: 'Available Tools',
          items: [
            { text: 'Overview', link: '/tools/' },
            { text: 'Test Flakiness Detector', link: '/tools/test-flakiness-detector/' },
            { text: 'CLI Progress Reporting', link: '/tools/cli-progress-reporting/' },
            { text: 'Cross-Platform Path Normalizer', link: '/tools/cross-platform-path-normalizer/' },
            { text: 'File-Based Semaphore', link: '/tools/file-based-semaphore/' },
            { text: 'Output Diffing Utility', link: '/tools/output-diffing-utility/' },
            { text: 'Structured Error Handler', link: '/tools/structured-error-handler/' }
          ]
        }
      ],
      '/guide/': [
        {
          text: 'Introduction',
          items: [
            { text: 'Philosophy', link: '/guide/philosophy' },
            { text: 'Principles', link: '/guide/principles' },
            { text: 'Getting Started', link: '/guide/getting-started' }
          ]
        },
        {
          text: 'Development',
          items: [
            { text: 'Quality Checklist', link: '/guide/quality-checklist' },
            { text: 'Testing Standards', link: '/guide/testing-standards' },
            { text: 'Security Guidelines', link: '/guide/security-guidelines' },
            { text: 'Contributing', link: '/guide/contributing' }
          ]
        }
      ],
      '/tools/test-flakiness-detector/': [
        {
          text: 'Test Flakiness Detector',
          items: [
            { text: 'Overview', link: '/tools/test-flakiness-detector/' },
            { text: 'Getting Started', link: '/tools/test-flakiness-detector/getting-started' },
            { text: 'CLI Usage', link: '/tools/test-flakiness-detector/cli-usage' },
            { text: 'Library Usage', link: '/tools/test-flakiness-detector/library-usage' },
            { text: 'Examples', link: '/tools/test-flakiness-detector/examples' },
            { text: 'API Reference', link: '/tools/test-flakiness-detector/api-reference' }
          ]
        }
      ],
      '/tools/cli-progress-reporting/': [
        {
          text: 'CLI Progress Reporting',
          items: [
            { text: 'Overview', link: '/tools/cli-progress-reporting/' },
            { text: 'Getting Started', link: '/tools/cli-progress-reporting/getting-started' },
            { text: 'CLI Usage', link: '/tools/cli-progress-reporting/cli-usage' },
            { text: 'Library Usage', link: '/tools/cli-progress-reporting/library-usage' },
            { text: 'Examples', link: '/tools/cli-progress-reporting/examples' },
            { text: 'Dogfooding', link: '/tools/cli-progress-reporting/dogfooding' },
            { text: 'API Reference', link: '/tools/cli-progress-reporting/api-reference' }
          ]
        }
      ],
      '/tools/cross-platform-path-normalizer/': [
        {
          text: 'Cross-Platform Path Normalizer',
          items: [
            { text: 'Overview', link: '/tools/cross-platform-path-normalizer/' },
            { text: 'Getting Started', link: '/tools/cross-platform-path-normalizer/getting-started' },
            { text: 'CLI Usage', link: '/tools/cross-platform-path-normalizer/cli-usage' },
            { text: 'Library Usage', link: '/tools/cross-platform-path-normalizer/library-usage' },
            { text: 'Examples', link: '/tools/cross-platform-path-normalizer/examples' },
            { text: 'API Reference', link: '/tools/cross-platform-path-normalizer/api-reference' }
          ]
        }
      ],
      '/tools/file-based-semaphore/': [
        {
          text: 'File-Based Semaphore',
          items: [
            { text: 'Overview', link: '/tools/file-based-semaphore/' },
            { text: 'Getting Started', link: '/tools/file-based-semaphore/getting-started' },
            { text: 'CLI Usage', link: '/tools/file-based-semaphore/cli-usage' },
            { text: 'Library Usage', link: '/tools/file-based-semaphore/library-usage' },
            { text: 'Examples', link: '/tools/file-based-semaphore/examples' },
            { text: 'Protocol Spec', link: '/tools/file-based-semaphore/protocol-spec' },
            { text: 'API Reference', link: '/tools/file-based-semaphore/api-reference' }
          ]
        }
      ],
      '/tools/output-diffing-utility/': [
        {
          text: 'Output Diffing Utility',
          items: [
            { text: 'Overview', link: '/tools/output-diffing-utility/' },
            { text: 'Getting Started', link: '/tools/output-diffing-utility/getting-started' },
            { text: 'CLI Usage', link: '/tools/output-diffing-utility/cli-usage' },
            { text: 'Library Usage', link: '/tools/output-diffing-utility/library-usage' },
            { text: 'Examples', link: '/tools/output-diffing-utility/examples' },
            { text: 'Output Formats', link: '/tools/output-diffing-utility/output-formats' },
            { text: 'API Reference', link: '/tools/output-diffing-utility/api-reference' }
          ]
        }
      ],
      '/tools/structured-error-handler/': [
        {
          text: 'Structured Error Handler',
          items: [
            { text: 'Overview', link: '/tools/structured-error-handler/' },
            { text: 'Getting Started', link: '/tools/structured-error-handler/getting-started' },
            { text: 'CLI Usage', link: '/tools/structured-error-handler/cli-usage' },
            { text: 'Library Usage', link: '/tools/structured-error-handler/library-usage' },
            { text: 'Examples', link: '/tools/structured-error-handler/examples' },
            { text: 'API Reference', link: '/tools/structured-error-handler/api-reference' }
          ]
        }
      ]
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/tuulbelt/tuulbelt' }
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2025 Tuulbelt'
    },

    search: {
      provider: 'local'
    }
  }
})
