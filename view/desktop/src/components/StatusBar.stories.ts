import type { Meta, StoryObj } from "@storybook/react";
import "@/assets/index.css";
import StatusBar from "@/components/StatusBar";

const meta = {
  title: "desktop/StatusBar",
  component: StatusBar,
  tags: ["autodocs"],
  parameters: {
    layout: "fullscreen",
  },
  args: {},
} satisfies Meta<typeof StatusBar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const WithBranch: Story = {
  args: {
    branch: "MOSSMVP-37-Backend-Migrate-existing-backend-in-Tauri",
  },
};

export const NoBranch: Story = {};
