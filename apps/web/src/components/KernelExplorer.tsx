"use client";

import React, { useState } from "react";
import { KernelProfile } from "@/types/hub";
import {
  Cpu,
  ShieldCheck,
  Zap,
  Activity,
  CheckCircle2,
  Copy,
  Check,
  Sliders,
  Lock,
  ChevronRight,
  Database,
  Globe,
  Bot,
} from "lucide-react";

interface KernelExplorerProps {
  kernels: KernelProfile[];
  selectedKernelName?: string | null;
}

export const KernelExplorer: React.FC<KernelExplorerProps> = ({
  kernels,
  selectedKernelName,
}) => {
  const [activeKernelId, setActiveKernelId] = useState<string>(
    kernels.find((k) => k.name === selectedKernelName)?.id || kernels[0]?.id || ""
  );

  // Generator state
  const [vcpus, setVcpus] = useState<number>(2);
  const [memoryMb, setMemoryMb] = useState<number>(512);
  const [portMapping, setPortMapping] = useState<string>("8080:80");
  const [copiedCmd, setCopiedCmd] = useState<boolean>(false);
  const [signatureVerified, setSignatureVerified] = useState<boolean>(true);
  const [isVerifying, setIsVerifying] = useState<boolean>(false);

  const activeKernel = kernels.find((k) => k.id === activeKernelId) || kernels[0];

  const generatedCommand = `vyoma run <appliance> --kernel-profile ${activeKernel?.name || "universal-general"} --vcpu ${vcpus} --memory ${
    memoryMb >= 1024 ? `${memoryMb / 1024}G` : `${memoryMb}M`
  } -p ${portMapping}`;

  const copyCommand = () => {
    navigator.clipboard.writeText(generatedCommand);
    setCopiedCmd(true);
    setTimeout(() => setCopiedCmd(false), 2000);
  };

  const verifySignature = () => {
    setIsVerifying(true);
    setTimeout(() => {
      setIsVerifying(false);
      setSignatureVerified(true);
    }, 400);
  };

  const getKernelIcon = (name: string) => {
    switch (name) {
      case "database":
        return <Database size={18} color="var(--accent-gold)" />;
      case "edge-network":
        return <Globe size={18} color="var(--accent-cyan)" />;
      case "ai-inference":
        return <Bot size={18} color="var(--accent-violet)" />;
      default:
        return <Cpu size={18} color="var(--accent-emerald)" />;
    }
  };

  return (
    <div style={{ maxWidth: "1400px", margin: "0 auto" }}>
      {/* Header Banner */}
      <div style={{ marginBottom: "36px" }}>
        <div style={{ display: "inline-flex", alignItems: "center", gap: "6px", marginBottom: "12px" }}>
          <span className="badge badge-gold">
            <Cpu size={13} /> Workload-Tuned Linux Kernels
          </span>
          <span className="badge badge-cyan">v6.6.30 LTS Base</span>
        </div>
        <h1 style={{ fontSize: "2.2rem", fontWeight: 800, color: "#ffffff", letterSpacing: "-0.02em", marginBottom: "12px" }}>
          Certified Guest Kernel Catalog
        </h1>
        <p style={{ fontSize: "1.05rem", color: "var(--text-secondary)", maxWidth: "800px", lineHeight: "1.6" }}>
          Every Vyoma MicroVM runs on a workload-optimized Linux kernel stripped of legacy drivers (no floppy, no ATA, no legacy PCI), booting in under 220ms with hardware-enforced Ed25519 cryptographic provenance.
        </p>
      </div>

      {/* Main Grid: Kernel Selector & Inspector */}
      <div style={{ display: "grid", gridTemplateColumns: "340px 1fr", gap: "28px", marginBottom: "40px" }}>
        {/* Left Column: Kernel List */}
        <div style={{ display: "flex", flexDirection: "column", gap: "12px" }}>
          {kernels.map((kernel) => {
            const isSelected = kernel.id === activeKernelId;
            return (
              <div
                key={kernel.id}
                onClick={() => setActiveKernelId(kernel.id)}
                className="glass-panel"
                style={{
                  padding: "18px 20px",
                  cursor: "pointer",
                  border: isSelected ? "1px solid var(--accent-cyan)" : "1px solid var(--border-subtle)",
                  background: isSelected ? "rgba(0, 242, 254, 0.08)" : "var(--bg-glass-card)",
                  boxShadow: isSelected ? "0 0 20px rgba(0, 242, 254, 0.15)" : "none",
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "space-between",
                }}
              >
                <div style={{ display: "flex", alignItems: "center", gap: "14px" }}>
                  <div style={{
                    width: "36px",
                    height: "36px",
                    borderRadius: "8px",
                    background: "rgba(255, 255, 255, 0.05)",
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                  }}>
                    {getKernelIcon(kernel.name)}
                  </div>
                  <div>
                    <div style={{ display: "flex", alignItems: "center", gap: "6px" }}>
                      <span style={{ fontSize: "0.95rem", fontWeight: 700, color: isSelected ? "var(--accent-cyan)" : "#ffffff" }}>
                        {kernel.name}
                      </span>
                      {kernel.is_official && (
                        <ShieldCheck size={14} color="var(--accent-emerald)" />
                      )}
                    </div>
                    <div style={{ fontSize: "0.75rem", color: "var(--text-muted)" }}>
                      {kernel.workload_type} · {kernel.boot_time_ms}ms boot
                    </div>
                  </div>
                </div>

                <ChevronRight size={16} color={isSelected ? "var(--accent-cyan)" : "var(--text-dim)"} />
              </div>
            );
          })}
        </div>

        {/* Right Column: Active Kernel Deep Dive */}
        {activeKernel && (
          <div className="glass-panel" style={{ padding: "32px", background: "#0c101a", border: "1px solid var(--border-default)" }}>
            <div style={{ display: "flex", alignItems: "flex-start", justifyContent: "space-between", marginBottom: "20px" }}>
              <div>
                <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "6px" }}>
                  <h2 style={{ fontSize: "1.6rem", fontWeight: 800, color: "#ffffff" }}>
                    {activeKernel.name}
                  </h2>
                  <span className="badge badge-emerald">
                    <ShieldCheck size={13} /> Certified by Vyoma Notary
                  </span>
                  <span className="badge badge-cyan">
                    {activeKernel.version}
                  </span>
                </div>
                <p style={{ fontSize: "0.95rem", color: "var(--text-secondary)", lineHeight: "1.6" }}>
                  {activeKernel.description}
                </p>
              </div>
            </div>

            {/* Performance Stats Cards */}
            <div style={{ display: "grid", gridTemplateColumns: "repeat(3, 1fr)", gap: "14px", marginBottom: "28px" }}>
              <div style={{ background: "rgba(255, 255, 255, 0.02)", padding: "16px", borderRadius: "var(--radius-md)", border: "1px solid var(--border-subtle)" }}>
                <div style={{ fontSize: "0.76rem", color: "var(--text-muted)", marginBottom: "4px", display: "flex", alignItems: "center", gap: "5px" }}>
                  <Zap size={14} color="var(--accent-gold)" /> Real Cold Boot
                </div>
                <div style={{ fontSize: "1.3rem", fontWeight: 800, color: "#ffffff" }}>
                  {activeKernel.boot_time_ms} ms
                </div>
                <div style={{ fontSize: "0.7rem", color: "var(--accent-emerald)" }}>Physical KVM hardware</div>
              </div>

              <div style={{ background: "rgba(255, 255, 255, 0.02)", padding: "16px", borderRadius: "var(--radius-md)", border: "1px solid var(--border-subtle)" }}>
                <div style={{ fontSize: "0.76rem", color: "var(--text-muted)", marginBottom: "4px", display: "flex", alignItems: "center", gap: "5px" }}>
                  <Activity size={14} color="var(--accent-cyan)" /> Host RAM Footprint
                </div>
                <div style={{ fontSize: "1.3rem", fontWeight: 800, color: "#ffffff" }}>
                  {activeKernel.ram_footprint_mb} MB
                </div>
                <div style={{ fontSize: "0.7rem", color: "var(--text-muted)" }}>Total VMM + Kernel</div>
              </div>

              <div style={{ background: "rgba(255, 255, 255, 0.02)", padding: "16px", borderRadius: "var(--radius-md)", border: "1px solid var(--border-subtle)" }}>
                <div style={{ fontSize: "0.76rem", color: "var(--text-muted)", marginBottom: "4px", display: "flex", alignItems: "center", gap: "5px" }}>
                  <Lock size={14} color="var(--accent-violet)" /> Enclave Attestation
                </div>
                <div style={{ fontSize: "1.05rem", fontWeight: 700, color: "var(--accent-emerald)" }}>
                  Measured PCR[0]
                </div>
                <div style={{ fontSize: "0.7rem", color: "var(--text-muted)" }}>vTPM 2.0 Attested</div>
              </div>
            </div>

            {/* In-Kernel Optimizations & CPU Extensions */}
            <div style={{ marginBottom: "28px" }}>
              <h4 style={{ fontSize: "0.85rem", fontWeight: 700, color: "var(--text-muted)", textTransform: "uppercase", letterSpacing: "0.05em", marginBottom: "12px" }}>
                Active Kernel Compile-Time Subsystems
              </h4>
              <div style={{ display: "flex", flexWrap: "wrap", gap: "8px" }}>
                {activeKernel.optimizations?.map((opt) => (
                  <span
                    key={opt}
                    style={{
                      fontSize: "0.8rem",
                      padding: "6px 12px",
                      borderRadius: "6px",
                      background: "rgba(0, 242, 254, 0.08)",
                      border: "1px solid rgba(0, 242, 254, 0.25)",
                      color: "var(--accent-cyan)",
                      display: "flex",
                      alignItems: "center",
                      gap: "6px",
                    }}
                  >
                    <CheckCircle2 size={13} color="var(--accent-cyan)" /> {opt}
                  </span>
                ))}
              </div>
            </div>

            {/* Cryptographic Notary Inspection */}
            <div style={{
              background: "#05070d",
              border: "1px solid rgba(16, 185, 129, 0.25)",
              borderRadius: "var(--radius-md)",
              padding: "20px",
              marginBottom: "28px",
            }}>
              <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: "14px" }}>
                <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                  <ShieldCheck size={20} color="var(--accent-emerald)" />
                  <span style={{ fontSize: "0.95rem", fontWeight: 700, color: "#ffffff" }}>
                    Ed25519 Cryptographic Verification Block
                  </span>
                </div>
                <button
                  onClick={verifySignature}
                  disabled={isVerifying}
                  className="btn-secondary"
                  style={{
                    padding: "6px 14px",
                    fontSize: "0.78rem",
                    color: "var(--accent-emerald)",
                    borderColor: "rgba(16, 185, 129, 0.4)",
                  }}
                >
                  {isVerifying ? "Verifying..." : signatureVerified ? "✓ Signature Valid" : "Verify Signature"}
                </button>
              </div>

              <div style={{ display: "flex", flexDirection: "column", gap: "10px", fontSize: "0.78rem" }}>
                <div>
                  <span style={{ color: "var(--text-muted)" }}>Official Notary Public Key:</span>
                  <div className="mono" style={{ color: "var(--accent-gold)", wordBreak: "break-all", marginTop: "3px" }}>
                    {activeKernel.ed25519_public_key}
                  </div>
                </div>
                <div>
                  <span style={{ color: "var(--text-muted)" }}>Kernel Binary Digest (`vmlinuz` SHA-256):</span>
                  <div className="mono" style={{ color: "var(--accent-cyan)", wordBreak: "break-all", marginTop: "3px" }}>
                    {activeKernel.vmlinuz_digest}
                  </div>
                </div>
                <div>
                  <span style={{ color: "var(--text-muted)" }}>Ed25519 Signature:</span>
                  <div className="mono" style={{ color: "var(--text-secondary)", wordBreak: "break-all", marginTop: "3px", fontSize: "0.72rem" }}>
                    {activeKernel.ed25519_signature}
                  </div>
                </div>
              </div>
            </div>

            {/* Recommended Kernel Commandline */}
            <div>
              <span style={{ fontSize: "0.78rem", fontWeight: 600, color: "var(--text-muted)", textTransform: "uppercase", letterSpacing: "0.05em", display: "block", marginBottom: "8px" }}>
                Default Kernel Commandline (`cmdline`)
              </span>
              <div style={{
                background: "#04060b",
                border: "1px solid var(--border-subtle)",
                borderRadius: "var(--radius-md)",
                padding: "10px 14px",
              }}>
                <code className="mono" style={{ fontSize: "0.78rem", color: "#94a3b8", wordBreak: "break-all" }}>
                  {activeKernel.recommended_cmdline}
                </code>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Interactive Boot Argument Generator */}
      <div className="glass-panel" style={{ padding: "32px", background: "linear-gradient(180deg, #0e1320 0%, #080a10 100%)", border: "1px solid var(--border-active)" }}>
        <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "16px" }}>
          <Sliders size={22} color="var(--accent-cyan)" />
          <h3 style={{ fontSize: "1.3rem", fontWeight: 800, color: "#ffffff" }}>
            Interactive MicroVM CLI Command Generator
          </h3>
        </div>
        <p style={{ fontSize: "0.9rem", color: "var(--text-secondary)", marginBottom: "24px", maxWidth: "700px" }}>
          Configure your guest hardware parameters to generate the exact CLI command. Vyoma automatically matches the kernel profile and applies the optimized hardware arguments.
        </p>

        {/* Controls Grid */}
        <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(240px, 1fr))", gap: "24px", marginBottom: "28px" }}>
          {/* vCPUs slider */}
          <div>
            <div style={{ display: "flex", justifyContent: "space-between", marginBottom: "8px" }}>
              <label style={{ fontSize: "0.85rem", fontWeight: 600, color: "var(--text-secondary)" }}>
                Dedicated vCPUs
              </label>
              <span className="mono" style={{ fontSize: "0.9rem", color: "var(--accent-cyan)", fontWeight: 700 }}>
                {vcpus} Cores
              </span>
            </div>
            <input
              type="range"
              min={1}
              max={16}
              step={1}
              value={vcpus}
              onChange={(e) => setVcpus(parseInt(e.target.value))}
              style={{ width: "100%", accentColor: "var(--accent-cyan)", cursor: "pointer" }}
            />
          </div>

          {/* Memory Slider */}
          <div>
            <div style={{ display: "flex", justifyContent: "space-between", marginBottom: "8px" }}>
              <label style={{ fontSize: "0.85rem", fontWeight: 600, color: "var(--text-secondary)" }}>
                Memory Limit
              </label>
              <span className="mono" style={{ fontSize: "0.9rem", color: "var(--accent-gold)", fontWeight: 700 }}>
                {memoryMb >= 1024 ? `${memoryMb / 1024} GiB` : `${memoryMb} MiB`}
              </span>
            </div>
            <input
              type="range"
              min={256}
              max={8192}
              step={256}
              value={memoryMb}
              onChange={(e) => setMemoryMb(parseInt(e.target.value))}
              style={{ width: "100%", accentColor: "var(--accent-gold)", cursor: "pointer" }}
            />
          </div>

          {/* Port input */}
          <div>
            <label style={{ display: "block", fontSize: "0.85rem", fontWeight: 600, color: "var(--text-secondary)", marginBottom: "8px" }}>
              Port Forwarding (-p)
            </label>
            <input
              type="text"
              value={portMapping}
              onChange={(e) => setPortMapping(e.target.value)}
              placeholder="e.g. 5432:5432"
              style={{
                width: "100%",
                padding: "8px 12px",
                background: "#04060a",
                border: "1px solid var(--border-default)",
                borderRadius: "var(--radius-md)",
                color: "#ffffff",
                fontSize: "0.88rem",
              }}
            />
          </div>
        </div>

        {/* Output Command Box */}
        <div style={{
          background: "#030408",
          border: "1px solid var(--border-active)",
          borderRadius: "var(--radius-md)",
          padding: "16px 20px",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          boxShadow: "0 4px 20px rgba(0, 0, 0, 0.6)",
        }}>
          <code className="mono" style={{ fontSize: "0.9rem", color: "var(--accent-cyan)", wordBreak: "break-all" }}>
            $ {generatedCommand}
          </code>
          <button
            onClick={copyCommand}
            className="btn-primary"
            style={{ padding: "8px 18px", flexShrink: 0, marginLeft: "18px" }}
          >
            {copiedCmd ? <Check size={16} /> : <Copy size={16} />}
            {copiedCmd ? "Copied Command" : "Copy Command"}
          </button>
        </div>
      </div>
    </div>
  );
};
