import { defineConfig } from 'vitepress';

// https://vitepress.dev/reference/site-config
export default defineConfig({
  outDir: './dist',
  title: 'Blue',
  description: 'Fast and extensible workspace manager',
  head: [['link', { rel: 'icon', href: '/favicon.ico' }]],
  cleanUrls: true,
  ignoreDeadLinks: true,
  themeConfig: {
    logo: '/logo.png',
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Getting Started', link: '/installation' },
    ],
    sidebar: [
      {
        text: 'Getting Started',
        items: [
          { text: 'Introduction', link: '/introduction' },
          { text: 'Installation', link: '/installation' },
          { text: 'Features', link: '/features' },
        ],
      },
      {
        text: 'Commands',
        items: [
          {
            collapsed: true,
            text: 'Blue',
            items: [
              { text: 'blue bin', link: '/commands/bin' },
              { text: 'blue update', link: '/commands/update' },
              { text: 'blue version', link: '/commands/version' },
            ],
          },
          {
            collapsed: true,
            text: 'Global',
            items: [{ text: 'blue touch', link: '/commands/touch' }],
          },
          {
            collapsed: true,
            text: 'Workspace',
            items: [
              { text: 'blue add', link: '/commands/add' },
              { text: 'blue analyze', link: '/commands/analyze' },
              { text: 'blue check', link: '/commands/check' },
              { text: 'blue clean', link: '/commands/clean' },
              { text: 'blue docker', link: '/commands/docker' },
              { text: 'blue fix', link: '/commands/fix' },
              { text: 'blue init', link: '/commands/init' },
              { text: 'blue run', link: '/commands/run' },
              { text: 'blue setup', link: '/commands/setup' },
            ],
          },
          {
            collapsed: true,
            text: 'Misc',
            items: [{ text: 'blue set', link: '/commands/set' }],
          },
        ],
      },
      {
        text: 'Automation',
        items: [
          {
            text: 'GitHub Actions',
            link: 'automation/github-actions',
          },
        ],
      },
    ],

    socialLinks: [{ icon: 'github', link: 'https://github.com/slekup/blue' }],
  },
});
