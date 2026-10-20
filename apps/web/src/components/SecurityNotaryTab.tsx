"use client";

import React, { useState } from "react";
import {
  ShieldCheck,
  Cpu,
  Lock,
  Network,
  UserCheck,
  FileCode,
  CheckCircle2,
} from "lucide-react";

export const SecurityNotaryTab: React.FC = () => {
  const [activePillar, setActivePillar] = useState<number>(1);

  const pillars = [
    {
      id: 1,
      title: "Pillar 1: Hardware-Enforced MicroVM Isolation",
      subtitle: "Dedicated Linux Kernel & KVM Boundary",
      icon: <Cpu size={24} color="var(--accent-cyan)" />,
      badge: "Immune to Container Escapes",
      description:
        "Unlike Docker containers which share the host kernel syscall interface, every Vyoma MicroVM executes inside its own independent Linux guest kernel via Cloud Hypervisor and Linux KVM. A total guest kernel crash or zero-day exploit cannot touch the host.",
      highlights: [
        "Independent syscall boundary (100% separate address space)",
        "Virtio-only device model (virtio-blk, net, fs, vsock) with no legacy hardware emulation",
        "Hardware Intel VT-x and AMD-V virtualization extensions enforce memory confidentiality",
      ],
      specRef: "docs/specifications/technical-spec-v2.md#pillar-1",
    },
    {
      id: 2,
      title: "Pillar 2: Least-Privilege Host Daemon",
      subtitle: "Strict Privilege Dropping (`privdrop.rs`)",
      icon: <UserCheck size={24} color="var(--accent-emerald)" />,
      badge: "Root-to-Unprivileged Drop",
      description:
        "The Vyoma daemon (`vyomad`) drops root privileges immediately upon socket and interface acquisition. It transitions to an unprivileged `vyoma` service user and strips all ambient capabilities down to only `CAP_NET_ADMIN` and `CAP_SYS_ADMIN`.",
      highlights: [
        "Immediate dropping of UID 0 / root to dedicated system user `vyoma`",
        "Linux capability bounding sets prevent host privilege escalation",
        "Systemd cgroup isolation limits CPU and memory exhaustion attacks",
      ],
      specRef: "crates/vyomad/src/privdrop.rs",
    },
    {
      id: 3,
      title: "Pillar 3: Measured Boot & Cryptographic Attestation",
      subtitle: "vTPM2 PCR Measurement & Ed25519 Signatures",
      icon: <Lock size={24} color="var(--accent-gold)" />,
      badge: "Zero-Trust Notary Supply Chain",
      description:
        "Every official image and kernel binary is cryptographically signed using Ed25519 keys and counter-signed by the Vyoma Root Notary Authority. Cloud Hypervisor measures the kernel and boot arguments into virtual TPM2 PCR[0] registers before CPU initialization.",
      highlights: [
        "Ed25519 digital signatures validated before any layer is mounted or booted",
        "vTPM2 quote generation enables remote verifiable hardware attestation",
        "Append-only Merkle tree transparency log prevents supply chain rollbacks",
      ],
      specRef: "crates/vyoma-image/src/signing.rs",
    },
    {
      id: 4,
      title: "Pillar 4: Isolated Network Mesh & Control Plane",
      subtitle: "Per-VM TAP Namespaces & WireGuard Overlays",
      icon: <Network size={24} color="var(--accent-violet)" />,
      badge: "VXLAN + WireGuard Encrypted",
      description:
        "Every MicroVM receives an isolated network namespace (netns) with dedicated TAP devices. Inter-node cluster communication runs across VXLAN overlays with automatic WireGuard peer-to-peer encryption for untrusted WAN links.",
      highlights: [
        "Hardware-isolated TAP devices with strict IP anti-spoofing eBPF rules",
        "Raft-coordinated swarm discovery automatically assigns private VXLAN VNI ranges",
        "Capability-based Bearer tokens protect all REST and gRPC daemon endpoints",
      ],
      specRef: "crates/vyoma-net/src/manager.rs",
    },
  ];

  const transparencyLog = [
    {
      timestamp: "2026-10-20 12:35:10 UTC",
      target: "kernel:database@6.6.30-vyoma",
      signer: "Vyoma Root Authority (239b0c...)",
      digest: "sha256:7a8b9c0d1e2f3a4b...",
      status: "Recorded in Merkle Tree #1042",
    },
    {
      timestamp: "2026-10-20 12:35:10 UTC",
      target: "appliance:vyoma/postgres@16",
      signer: "Verified Publisher Notary",
      digest: "sha256:f290d8a1c97e14f8...",
      status: "Recorded in Merkle Tree #1041",
    },
    {
      timestamp: "2026-10-20 12:35:10 UTC",
      target: "kernel:ai-inference@6.6.30-vyoma",
      signer: "Vyoma Root Authority (239b0c...)",
      digest: "sha256:9c0d1e2f3a4b5c6d...",
      status: "Recorded in Merkle Tree #1040",
    },
    {
      timestamp: "2026-10-20 12:35:10 UTC",
      target: "appliance:vyoma/ollama@latest",
      signer: "Verified Publisher Notary",
      digest: "sha256:b8472910fa37e204...",
      status: "Recorded in Merkle Tree #1039",
    },
  ];

  return (
    <div style={{ maxWidth: "1400px", margin: "0 auto" }}>
      {/* Header */}
      <div style={{ marginBottom: "36px" }}>
        <div style={{ display: "inline-flex", alignItems: "center", gap: "6px", marginBottom: "12px" }}>
          <span className="badge badge-emerald">
            <ShieldCheck size={13} /> Supply Chain Security
          </span>
          <span className="badge badge-cyan">Zero Container Escapes</span>
        </div>
        <h1 style={{ fontSize: "2.2rem", fontWeight: 800, color: "#ffffff", letterSpacing: "-0.02em", marginBottom: "12px" }}>
          The 4 Enterprise Security Pillars
        </h1>
        <p style={{ fontSize: "1.05rem", color: "var(--text-secondary)", maxWidth: "800px", lineHeight: "1.6" }}>
          Vyoma rejects marketing fluff. Rather than counting superficial checks as separate layers, Vyoma’s architecture is built on four verifiable, hardware-enforced pillars of isolation and provenance.
        </p>
      </div>

      {/* 4 Pillars Interactive Layout */}
      <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(280px, 1fr))", gap: "20px", marginBottom: "40px" }}>
        {pillars.map((pillar) => {
          const isSelected = activePillar === pillar.id;
          return (
            <div
              key={pillar.id}
              onClick={() => setActivePillar(pillar.id)}
              className="glass-panel"
              style={{
                padding: "24px",
                cursor: "pointer",
                border: isSelected ? "1px solid var(--accent-cyan)" : "1px solid var(--border-subtle)",
                background: isSelected ? "rgba(0, 242, 254, 0.08)" : "var(--bg-glass-card)",
                boxShadow: isSelected ? "0 0 24px rgba(0, 242, 254, 0.15)" : "none",
                display: "flex",
                flexDirection: "column",
                justifyContent: "space-between",
              }}
            >
              <div>
                <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: "16px" }}>
                  <div style={{
                    width: "44px",
                    height: "44px",
                    borderRadius: "10px",
                    background: "rgba(255, 255, 255, 0.05)",
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                  }}>
                    {pillar.icon}
                  </div>
                  <span className="badge badge-cyan" style={{ fontSize: "0.68rem" }}>
                    {pillar.badge}
                  </span>
                </div>

                <h3 style={{ fontSize: "1.1rem", fontWeight: 700, color: "#ffffff", marginBottom: "6px" }}>
                  {pillar.title}
                </h3>
                <div style={{ fontSize: "0.8rem", color: "var(--accent-cyan)", fontWeight: 600, marginBottom: "14px" }}>
                  {pillar.subtitle}
                </div>
                <p style={{ fontSize: "0.85rem", color: "var(--text-secondary)", lineHeight: "1.5" }}>
                  {pillar.description}
                </p>
              </div>

              <div style={{ marginTop: "20px", paddingTop: "14px", borderTop: "1px solid var(--border-subtle)" }}>
                <span style={{ fontSize: "0.75rem", color: "var(--accent-cyan)", fontWeight: 600 }}>
                  {isSelected ? "● Active Deep Dive" : "Click to view architectural highlights →"}
                </span>
              </div>
            </div>
          );
        })}
      </div>

      {/* Selected Pillar Technical Highlights */}
      {(() => {
        const selected = pillars.find((p) => p.id === activePillar);
        if (!selected) return null;
        return (
          <div className="glass-panel" style={{ padding: "32px", background: "#0c101a", border: "1px solid var(--border-default)", marginBottom: "40px" }}>
            <div style={{ display: "flex", alignItems: "center", gap: "12px", marginBottom: "16px" }}>
              {selected.icon}
              <h3 style={{ fontSize: "1.3rem", fontWeight: 800, color: "#ffffff" }}>
                Technical Specification: {selected.title}
              </h3>
            </div>
            <p style={{ fontSize: "0.95rem", color: "var(--text-secondary)", marginBottom: "20px", lineHeight: "1.6" }}>
              {selected.description}
            </p>

            <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(280px, 1fr))", gap: "14px", marginBottom: "24px" }}>
              {selected.highlights.map((h, i) => (
                <div
                  key={i}
                  style={{
                    background: "rgba(255, 255, 255, 0.02)",
                    border: "1px solid var(--border-subtle)",
                    borderRadius: "var(--radius-md)",
                    padding: "16px",
                    display: "flex",
                    alignItems: "flex-start",
                    gap: "10px",
                  }}
                >
                  <CheckCircle2 size={16} color="var(--accent-emerald)" style={{ flexShrink: 0, marginTop: "2px" }} />
                  <span style={{ fontSize: "0.85rem", color: "#e2e8f0", lineHeight: "1.5" }}>
                    {h}
                  </span>
                </div>
              ))}
            </div>

            <div style={{ display: "flex", alignItems: "center", gap: "8px", fontSize: "0.78rem", color: "var(--text-muted)" }}>
              <span>Source Implementation:</span>
              <code className="mono" style={{ color: "var(--accent-cyan)", background: "rgba(0, 0, 0, 0.4)", padding: "2px 8px", borderRadius: "4px" }}>
                {selected.specRef}
              </code>
            </div>
          </div>
        );
      })()}

      {/* Merkle Tree Transparency Log */}
      <div className="glass-panel" style={{ padding: "32px", background: "#06080e", border: "1px solid var(--border-subtle)" }}>
        <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: "16px" }}>
          <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
            <FileCode size={20} color="var(--accent-gold)" />
            <h3 style={{ fontSize: "1.2rem", fontWeight: 800, color: "#ffffff" }}>
              Append-Only Notary Transparency Log
            </h3>
          </div>
          <span className="badge badge-emerald">
            Cryptographically Immutable
          </span>
        </div>
        <p style={{ fontSize: "0.88rem", color: "var(--text-secondary)", marginBottom: "20px" }}>
          Every official image push, signature verification, and kernel release generates a deterministic cryptographic entry in the public Merkle tree log.
        </p>

        <div style={{ display: "flex", flexDirection: "column", gap: "10px" }}>
          {transparencyLog.map((entry, idx) => (
            <div
              key={idx}
              style={{
                background: "rgba(255, 255, 255, 0.02)",
                border: "1px solid var(--border-subtle)",
                borderRadius: "var(--radius-md)",
                padding: "14px 18px",
                display: "flex",
                alignItems: "center",
                justifyContent: "space-between",
                flexWrap: "wrap",
                gap: "12px",
                fontSize: "0.82rem",
              }}
            >
              <div style={{ display: "flex", alignItems: "center", gap: "12px" }}>
                <span className="mono" style={{ color: "var(--text-muted)" }}>{entry.timestamp}</span>
                <span className="mono" style={{ color: "var(--accent-cyan)", fontWeight: 700 }}>{entry.target}</span>
              </div>
              <div style={{ display: "flex", alignItems: "center", gap: "16px" }}>
                <span className="mono" style={{ color: "var(--text-secondary)" }}>{entry.digest}</span>
                <span className="badge badge-cyan" style={{ fontSize: "0.72rem" }}>{entry.status}</span>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
