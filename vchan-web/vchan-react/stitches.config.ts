import { createStitches } from '@stitches/react';

export const {
    styled,
    css,
    globalCss,
    keyframes,
    getCssText,
    theme,
    createTheme,
    config,
  } = createStitches({
    theme: {
      colors: {
        vc_main: '#c44e4e',
        vc_sub: '#243085',
        vc_surface: '#f7f0df',
        vc_dark: '#142035',
        vc_gold: '#ccaa33',
      },
    },
    media: {
      bp1: '(min-width: 480px)',
    },
    utils: {
      marginX: (value: any) => ({ marginLeft: value, marginRight: value }),
    },
  });
  