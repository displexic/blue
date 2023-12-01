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
          { text: 'Installation', link: '/installation' },
          { text: 'Features', link: '/features' },
        ],
      },
      {
        text: 'Commands',
        items: [
          { text: 'add', link: '/commands/add' },
          { text: 'analyze', link: '/commands/analyze' },
          { text: 'bin', link: '/commands/bin' },
          { text: 'check', link: '/commands/check' },
          { text: 'clean', link: '/commands/clean' },
          { text: 'docker', link: '/commands/docker' },
          { text: 'fix', link: '/commands/fix' },
          { text: 'init', link: '/commands/init' },
          { text: 'run', link: '/commands/run' },
          { text: 'setup', link: '/commands/setup' },
          { text: 'set', link: '/commands/set' },
          { text: 'touch', link: '/commands/touch' },
          { text: 'update', link: '/commands/update' },
          { text: 'version', link: '/commands/version' },
        ],
      },
    ],

    socialLinks: [{ icon: 'github', link: 'https://github.com/slekup/blue' }],
  },
});
