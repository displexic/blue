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
          { text: 'Argument Syntax', link: '/commands/argument-syntax' },
          {
            collapsed: true,
            text: 'Blue',
            items: [
              { text: 'blue bin', link: '/commands/blue/bin' },
              { text: 'blue bootstrap', link: '/commands/blue/bootstrap' },
              { text: 'blue update', link: '/commands/blue/update' },
              { text: 'blue version', link: '/commands/blue/version' },
            ],
          },
          {
            collapsed: true,
            text: 'Global',
            items: [{ text: 'blue touch', link: '/commands/global/touch' }],
          },
          {
            collapsed: true,
            text: 'Workspace',
            items: [
              { text: 'blue add', link: '/commands/workspace/add' },
              { text: 'blue analyze', link: '/commands/workspace/analyze' },
              { text: 'blue check', link: '/commands/workspace/check' },
              { text: 'blue clean', link: '/commands/workspace/clean' },
              { text: 'blue docker', link: '/commands/workspace/docker' },
              { text: 'blue fix', link: '/commands/workspace/fix' },
              { text: 'blue init', link: '/commands/workspace/init' },
              { text: 'blue run', link: '/commands/workspace/run' },
              { text: 'blue setup', link: '/commands/workspace/setup' },
            ],
          },
          {
            collapsed: true,
            text: 'Misc',
            items: [{ text: 'blue set', link: '/commands/misc/set' }],
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
