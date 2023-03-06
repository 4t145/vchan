import { styled, theme } from "../../stitches.config";

export const VcrButton = styled("button", {
  backgroundColor: "$vc_surface",
  color: "black",
  borderRadius: 4,
  border: "1px solid transparent",
  padding: " 0.4em 0.6em",
  margin: "4px",
  fontSize: "1em",
  fontWeight: "500",
  fontFamily: "inherit",
  cursor: "pointer",
  transition: "box-shadow 0.25s, transform 0.25s",
  boxShadow: `4px 4px ${theme.colors.vc_main}`,
  type: "button",
  "&:hover": {
    boxShadow: `2px 2px ${theme.colors.vc_main}, -2px -2px ${theme.colors.vc_sub}`,
  },
  "&:active": {
    boxShadow: `-4px -4px ${theme.colors.vc_sub}`,
  },
  "&:focus": {
    // transform: "scale(1.1);",
  },

  '& svg': {
    display: 'inline-block',
    verticalAlign: 'middle',
    height: '100%',
    marginX: '3px',
  },

  variants: {
    fill: {
      true: {
        height: "100%"
      }
    }
  }
});
