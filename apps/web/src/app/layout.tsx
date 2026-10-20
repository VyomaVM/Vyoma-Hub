import type { Metadata, Viewport } from "next";
import "./globals.css";

export const viewport: Viewport = {
  width: "device-width",
  initialScale: 1,
};

export const metadata: Metadata = {
  title: "Vyoma Hub — Official App Store & Registry for Micro-OS Appliances & Certified Kernels",
  description:
    "Official App Store for hardware-isolated micro-OS appliances and certified Linux kernels for Vyoma MicroVMs. Booting in <250ms with Ed25519 cryptographic provenance.",
  keywords: [
    "vyoma",
    "microvm",
    "cloud hypervisor",
    "kernel catalog",
    "vmif",
    "appliance store",
    "oci registry",
    "hardware isolation",
  ],
  authors: [{ name: "Vyoma Team" }],
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
