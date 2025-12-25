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
            { text: 'File-Based Semaphore', link: '/tools/file-based-semaphore/' }
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
