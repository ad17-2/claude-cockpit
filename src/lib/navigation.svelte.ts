import {
  LayoutDashboard,
  Activity,
  FileText,
  Settings,
  Bot,
  MessageSquare,
} from "lucide-svelte";

type LucideIcon = typeof LayoutDashboard;

export interface NavItem {
  label: string;
  href: string;
  icon: LucideIcon;
}

export const navItems: NavItem[] = [
  { label: "Dashboard", href: "/", icon: LayoutDashboard },
  { label: "Sessions", href: "/sessions", icon: Activity },
  { label: "CLAUDE.md", href: "/claude-md", icon: FileText },
  { label: "Settings", href: "/settings", icon: Settings },
  { label: "Entities", href: "/entities", icon: Bot },
  { label: "History", href: "/history", icon: MessageSquare },
];
