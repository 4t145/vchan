import * as NavigationMenu from "@radix-ui/react-navigation-menu";
import { clsx } from "clsx";


export function HomeMenu() {
  const itemClassName = clsx(
    "bg-surface text-black w-full flex justify-left shadow-main p-4",
    "text-sm font-medium",
    "focus:outline-none focus-visible:ring focus-visible:ring-purple-500 focus-visible:ring-opacity-75"
  );
  const classItem = clsx(
    "bg-surface text-black w-full flex justify-left shadow-main p-4",
    "text-sm font-medium",
    "focus:outline-none focus-visible:ring focus-visible:ring-purple-500 focus-visible:ring-opacity-75",
    "hover:transition-all"
  );

  return (
    <>
      <NavigationMenu.Root orientation="vertical" className="relative">
        <NavigationMenu.List className="flex flex-col p-2 space-y-2 w-32 h-12">
          <NavigationMenu.Item>
            <NavigationMenu.Trigger className={classItem}>
              主页
            </NavigationMenu.Trigger>
          </NavigationMenu.Item>
          <NavigationMenu.Item>
            <NavigationMenu.Trigger className={classItem}>
              板块
            </NavigationMenu.Trigger>
            <NavigationMenu.Content>
              <NavigationMenu.Sub
                orientation="vertical"
                className={clsx(
                  "radix-motion-from-start:animate-enter-from-top",
                  "radix-motion-from-end:animate-enter-from-top",
                  "radix-motion-to-start:animate-exit-to-top",
                  "radix-motion-to-end:animate-exit-to-top"
                )}
              >
                <NavigationMenu.List className="flex flex-col space-y-2 mr-4">
                  <NavigationMenu.Item className={classItem}>
                    <NavigationMenu.Trigger onClick={()=>{
                      // router.push('/board/综合')
                    }}>综合</NavigationMenu.Trigger>
                  </NavigationMenu.Item>
                </NavigationMenu.List>
              </NavigationMenu.Sub>
            </NavigationMenu.Content>
          </NavigationMenu.Item>
        </NavigationMenu.List>
      </NavigationMenu.Root>
    </>
  );
}
