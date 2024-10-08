import type { Meta, StoryObj } from "@storybook/react";

// api reference https://www.radix-ui.com/primitives/docs/components/dropdown-menu

import {
  DropdownMenu,
  DropdownMenuArrow,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "./DropdownMenu";

import { useState } from "react";

const meta: Meta<typeof DropdownMenu> = {
  title: "Shared/DropdownMenu",
  component: DropdownMenu,
  parameters: {
    design: [
      {
        name: "Dark",
        type: "figma",
        url: "https://www.figma.com/design/acKJvhO9lMOv9wVObplm0H/M?node-id=999-2495&t=1FSPmBu2CHEWPqld-4",
      },
      {
        name: "Light",
        type: "figma",
        url: "https://www.figma.com/design/acKJvhO9lMOv9wVObplm0H/M?node-id=1097-3377&t=1FSPmBu2CHEWPqld-4",
      },
    ],
  },
  tags: ["autodocs"],
  decorators: [
    (Story) => (
      <div className="flex justify-center">
        <Story />
      </div>
    ),
  ],
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <DropdownMenu>
      <DropdownMenuTrigger>Click me!</DropdownMenuTrigger>
      <DropdownMenuContent>
        <DropdownMenuGroup>
          <DropdownMenuItem icon="Home1">
            <span>Profile</span>
            <DropdownMenuShortcut>⇧⌘P</DropdownMenuShortcut>
          </DropdownMenuItem>
          <DropdownMenuItem icon="Issues">
            <span>Billing</span>
            <DropdownMenuShortcut>⌘B</DropdownMenuShortcut>
          </DropdownMenuItem>
          <DropdownMenuItem icon="Reports">
            <span>Settings</span>
            <DropdownMenuShortcut>⌘S</DropdownMenuShortcut>
          </DropdownMenuItem>
          <DropdownMenuItem icon="Search">
            <span>Keyboard shortcuts</span>
            <DropdownMenuShortcut>⌘K</DropdownMenuShortcut>
          </DropdownMenuItem>
        </DropdownMenuGroup>

        <DropdownMenuSeparator />

        <DropdownMenuGroup>
          <DropdownMenuItem icon="Code">Team</DropdownMenuItem>

          <DropdownMenuSub>
            <DropdownMenuSubTrigger icon="Code">
              <span>Invite users</span>
            </DropdownMenuSubTrigger>
            <DropdownMenuPortal>
              <DropdownMenuSubContent>
                <DropdownMenuItem icon="Home1">
                  <span>Email</span>
                </DropdownMenuItem>
                <DropdownMenuItem>
                  <span>Message</span>
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem icon="Home1">
                  <span>More...</span>
                </DropdownMenuItem>
              </DropdownMenuSubContent>
            </DropdownMenuPortal>
          </DropdownMenuSub>

          <DropdownMenuSub>
            <DropdownMenuSubTrigger icon="Home1">
              <span>New team</span>
            </DropdownMenuSubTrigger>
            <DropdownMenuSubContent>
              <DropdownMenuItem>
                <span>Email</span>
              </DropdownMenuItem>
              <DropdownMenuItem>
                <span>Message</span>
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem>
                <span>More...</span>
              </DropdownMenuItem>
            </DropdownMenuSubContent>
          </DropdownMenuSub>
        </DropdownMenuGroup>

        <DropdownMenuSeparator />

        <DropdownMenuItem>
          <a href="https://github.com/4rchr4y/moss">Moss GitHub</a>
        </DropdownMenuItem>
        <DropdownMenuItem>
          <span>Support</span>
        </DropdownMenuItem>
        <DropdownMenuItem disabled>
          <span>API</span>
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem icon="Home1">
          <span>Log out</span>
          <DropdownMenuShortcut>⇧⌘Q</DropdownMenuShortcut>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  ),
};

export const ExampleFromFigma: Story = {
  render: () => (
    <DropdownMenu>
      <DropdownMenuTrigger>Click me!</DropdownMenuTrigger>

      <DropdownMenuContent>
        <DropdownMenuSub>
          <DropdownMenuSubTrigger>
            <span>File</span>
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem icon="Home1">
              <span>More...</span>
            </DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>

        <DropdownMenuSub>
          <DropdownMenuSubTrigger>
            <span>Edit</span>
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem icon="Home1">
              <span>More...</span>
            </DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>

        <DropdownMenuSub>
          <DropdownMenuSubTrigger>
            <span>View</span>
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem icon="Home1">
              <span>More...</span>
            </DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>

        <DropdownMenuSub>
          <DropdownMenuSubTrigger>
            <span>Window</span>
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem icon="Home1">
              <span>More...</span>
            </DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>

        <DropdownMenuSeparator />

        <DropdownMenuItem>
          <svg width="18" height="18" viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
              d="M3.14961 8.32461H2.47461V6.61461C2.47461 6.11056 2.47461 5.85854 2.5727 5.66602C2.65899 5.49667 2.79667 5.35899 2.96602 5.2727C3.15854 5.17461 3.41056 5.17461 3.91461 5.17461H6.07461V4.04961C6.07461 3.17976 6.77976 2.47461 7.64961 2.47461C8.51946 2.47461 9.22461 3.17976 9.22461 4.04961V5.17461L11.3846 5.17461C11.8887 5.17461 12.1407 5.17461 12.3332 5.2727C12.5025 5.35899 12.6402 5.49667 12.7265 5.66602C12.8246 5.85854 12.8246 6.11056 12.8246 6.61461V8.77461H13.9496C14.8195 8.77461 15.5246 9.47976 15.5246 10.3496C15.5246 11.2195 14.8195 11.9246 13.9496 11.9246H12.8246V14.0846C12.8246 14.5887 12.8246 14.8407 12.7265 15.0332C12.6402 15.2025 12.5025 15.3402 12.3332 15.4265C12.1407 15.5246 11.8887 15.5246 11.3846 15.5246H9.67461V14.8496C9.67461 13.7312 8.76799 12.8246 7.64961 12.8246C6.53123 12.8246 5.62461 13.7312 5.62461 14.8496V15.5246H3.91461C3.41056 15.5246 3.15854 15.5246 2.96602 15.4265C2.79667 15.3402 2.65899 15.2025 2.5727 15.0332C2.47461 14.8407 2.47461 14.5887 2.47461 14.0846V12.3746H3.14961C4.26799 12.3746 5.17461 11.468 5.17461 10.3496C5.17461 9.23123 4.26799 8.32461 3.14961 8.32461Z"
              stroke="#FAFAFA"
              stroke-width="1.15"
              stroke-linejoin="round"
            />
          </svg>
          <span>Extensions</span>
          <DropdownMenuShortcut>⇧⌘E</DropdownMenuShortcut>
        </DropdownMenuItem>

        <DropdownMenuItem>
          <span>Help</span>
          <DropdownMenuShortcut>⇧⌘H</DropdownMenuShortcut>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  ),
};

export const Groups: Story = {
  render: () => (
    <DropdownMenu>
      <DropdownMenuTrigger>Click me!</DropdownMenuTrigger>

      <DropdownMenuContent>
        <DropdownMenuGroup>
          <DropdownMenuItem>Groups don't really add any styles or functionality :(</DropdownMenuItem>
        </DropdownMenuGroup>

        <DropdownMenuGroup>
          <DropdownMenuItem icon="Home1">1 - Group item with icon</DropdownMenuItem>
          <DropdownMenuItem>1 - Group item</DropdownMenuItem>
        </DropdownMenuGroup>

        <DropdownMenuGroup>
          <DropdownMenuItem icon="Home1">2 - Group item with icon</DropdownMenuItem>
          <DropdownMenuItem>2 - Group item</DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  ),
};

export const DropdownMenuSubExample: Story = {
  render: () => (
    <DropdownMenu>
      <DropdownMenuTrigger>Click me!</DropdownMenuTrigger>

      <DropdownMenuContent>
        <DropdownMenuSub>
          <DropdownMenuSubTrigger icon="Home1">Sub trigger with icon</DropdownMenuSubTrigger>

          <DropdownMenuSubContent>
            <DropdownMenuItem icon="Home1">More...</DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>

        <DropdownMenuSub>
          <DropdownMenuSubTrigger>Some content</DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem icon="Home1">More...</DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>

        <DropdownMenuSub>
          <DropdownMenuSubTrigger>More content</DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem icon="Home1">More...</DropdownMenuItem>
            <DropdownMenuItem icon="Home1">More...</DropdownMenuItem>
            <DropdownMenuItem icon="Home1">More...</DropdownMenuItem>
            <DropdownMenuItem icon="Home1">More...</DropdownMenuItem>
            <DropdownMenuItem icon="Home1">More...</DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>

        <DropdownMenuSub>
          <DropdownMenuSubTrigger>A lot more content</DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem icon="Home1">More...</DropdownMenuItem>

            <DropdownMenuItem icon="Home1">More... More</DropdownMenuItem>
            <DropdownMenuItem icon="Home1">More... More... More</DropdownMenuItem>
            <DropdownMenuItem icon="Home1">More... More... More... More...</DropdownMenuItem>
            <DropdownMenuItem icon="Home1">More... More... More... More... More...</DropdownMenuItem>
            <DropdownMenuItem icon="Home1">More... More... More... More... More... More...</DropdownMenuItem>
            <DropdownMenuItem icon="Home1">More... More... More... More... More... More... More</DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>
      </DropdownMenuContent>
    </DropdownMenu>
  ),
};

export const Checkboxes: Story = {
  render: () => {
    const [list1, setList1] = useState([
      { label: "Snap to geometry", checked: true },
      { label: "Snap to objects", checked: true },
      { label: "Snap to pixel grid", shortcut: "⇧⌘`", checked: true, disabled: false },
    ]);

    const [list2, setList2] = useState([
      { label: "Keep tool selected after use", checked: false },
      { label: "Highlight layers on hover", checked: true },
      { label: "Rename duplicated layers", checked: true },
      { label: "Show dimensions on objects", checked: true },
      { label: "Hide canvas UI during changes", checked: true },
      { label: "Substitute smart quotes", checked: true },
      { label: "Flip objects while resizing", checked: true },
      { label: "Keyboard zooms into selection", checked: false },
      { label: "Invert zoom direction", checked: false },
      { label: "Ctrl+click opens right click menus", checked: false },
      { label: "Use number keys for opacity", checked: true },
      { label: "Use old shortcuts for outlines", checked: false },
      { label: "Open links in desktop app", checked: false },
      { label: "This should be disabled", checked: false, disabled: true },
    ]);

    const [list3, setList3] = useState([
      { label: "Use scroll wheel zoom", checked: false },
      { label: "Right-click and drag to pan", checked: false, disabled: false },
    ]);

    return (
      <DropdownMenu>
        <DropdownMenuTrigger>Click me!</DropdownMenuTrigger>
        <DropdownMenuContent>
          {list1.map((item, index) => (
            <DropdownMenuCheckboxItem
              key={index}
              checked={item.checked}
              disabled={item.disabled}
              onCheckedChange={() => {
                setList1(list1.map((i, iIndex) => (iIndex === index ? { ...i, checked: !i.checked } : i)));
              }}
            >
              {item.label}
              {item.shortcut && <DropdownMenuShortcut>{item.shortcut}</DropdownMenuShortcut>}
            </DropdownMenuCheckboxItem>
          ))}

          <DropdownMenuSeparator />

          {list2.map((item, index) => (
            <DropdownMenuCheckboxItem
              key={index}
              checked={item.checked}
              disabled={item.disabled}
              onCheckedChange={() => {
                setList2(list2.map((i, iIndex) => (iIndex === index ? { ...i, checked: !i.checked } : i)));
              }}
            >
              {item.label}
            </DropdownMenuCheckboxItem>
          ))}

          <DropdownMenuSeparator />

          {list3.map((item, index) => (
            <DropdownMenuCheckboxItem
              key={index}
              checked={item.checked}
              disabled={item.disabled}
              onCheckedChange={() => {
                setList3(list3.map((i, iIndex) => (iIndex === index ? { ...i, checked: !i.checked } : i)));
              }}
            >
              {item.label}
            </DropdownMenuCheckboxItem>
          ))}

          <DropdownMenuSeparator />

          <DropdownMenuSub>
            <DropdownMenuSubTrigger>Theme</DropdownMenuSubTrigger>
            <DropdownMenuSubContent>123</DropdownMenuSubContent>
          </DropdownMenuSub>
          <DropdownMenuSub>
            <DropdownMenuSubTrigger>Labs</DropdownMenuSubTrigger>
            <DropdownMenuSubContent>123</DropdownMenuSubContent>
          </DropdownMenuSub>

          <DropdownMenuItem>Color profile...</DropdownMenuItem>
          <DropdownMenuItem>Keyboard layout...</DropdownMenuItem>
          <DropdownMenuItem>Nudge amount...</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    );
  },
};

export const Radio: Story = {
  render: () => {
    const [radioGroup1Value, setRadioGroup1Value] = useState<string | undefined>(undefined);
    const [radioGroup2Value, setRadioGroup2Value] = useState<string | undefined>(undefined);

    const radioGroup1 = [
      {
        id: 1,
        label: "radio1",
      },
      {
        id: 2,
        label: "radio2",
      },
      {
        id: 3,
        label: "radio3",
      },
      {
        id: 4,
        label: "radio disabled",
        disabled: true,
      },
    ];

    const radioGroup2 = [
      {
        id: 1,
        label: "another radio 1",
      },
      {
        id: 2,
        label: "another radio 2",
      },
      {
        id: 3,
        label: "another radio 3",
      },
      {
        id: 4,
        label: "another radio disabled",
        disabled: true,
      },
    ];

    return (
      <DropdownMenu>
        <DropdownMenuTrigger>Click me!</DropdownMenuTrigger>

        <DropdownMenuContent>
          <DropdownMenuRadioGroup value={radioGroup1Value} onValueChange={setRadioGroup1Value}>
            {radioGroup1.map((item) => (
              <DropdownMenuRadioItem
                value={item.label}
                key={item.id}
                disabled={item.disabled}
                onSelect={(e) => {
                  e.preventDefault();
                }}
              >
                {item.label}
              </DropdownMenuRadioItem>
            ))}
          </DropdownMenuRadioGroup>

          <DropdownMenuSeparator />

          <DropdownMenuRadioGroup value={radioGroup2Value} onValueChange={setRadioGroup2Value}>
            {radioGroup2.map((item) => (
              <DropdownMenuRadioItem
                value={item.label}
                key={item.id}
                disabled={item.disabled}
                onSelect={(e) => {
                  e.preventDefault();
                }}
              >
                {item.label}
              </DropdownMenuRadioItem>
            ))}
          </DropdownMenuRadioGroup>
        </DropdownMenuContent>
      </DropdownMenu>
    );
  },
};
