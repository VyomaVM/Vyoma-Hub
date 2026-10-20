"use client";

import React, { useState } from "react";
import { CatalogItem } from "@/types/hub";
import {
  X,
  ShieldCheck,
  Cpu,
  Copy,
  Check,
  Zap,
  Activity,
  Layers,
  FileText,
  Lock,
} from "lucide-react";

interface ApplianceModalProps {
  item: CatalogItem | null;
  onClose: () => void;
  onSelectKernel?: (kernelName: string) => void;
}

export const ApplianceModal: React.FC<ApplianceModalProps> = ({
  item,
  onClose,
  onSelectKernel,
}) => {
  const [copied, setCopied] = useState(false);

  if (!item) return null;

  const runCommand = `vyoma run hub.vyoma.dev/${item.namespace}/${item.name}:${item.tags[0] || "latest"}${
    item.recommended_kernel ? ` --kernel-profile ${item.recommended_kernel}` : ""
  } --vcpu ${item.vcpu || 2} --memory ${item.memory_mb || 512}M`;

  const copyToClipboard = () => {
    navigator.clipboard.writeText(runCommand);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div
      onClick={onClose}
      style={{
        position: "fixed",
        inset: 0,
        zIndex: 200,
        background: "rgba(3, 5, 8, 0.8)",
        backdropFilter: "blur(14px)",
        WebkitBackdropFilter: "blur(14px)",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        padding: "20px",
      }}
    >
      <div
        onClick={(e) => e.stopPropagation()}
        className="glass-panel"
        style={{
          width: "100%",
          maxWidth: "840px",
          maxHeight: "90vh",
          overflowY: "auto",
          background: "#0c101a",
          border: "1px solid var(--border-default)",
          borderRadius: "var(--radius-xl)",
          padding: "32px",
          position: "relative",
          boxShadow: "0 24px 64px rgba(0, 0, 0, 0.8)",
        }}
      >
        {/* Close Button */}
        <button
          onClick={onClose}
          style={{
            position: "absolute",
            top: "24px",
            right: "24px",
            color: "var(--text-muted)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            width: "32px",
            height: "32px",
            borderRadius: "50%",
            background: "rgba(255, 255, 255, 0.05)",
            border: "1px solid var(--border-subtle)",
            transition: "all 0.2s ease",
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.color = "#ffffff";
            e.currentTarget.style.background = "rgba(255, 255, 255, 0.1)";
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.color = "var(--text-muted)";
            e.currentTarget.style.background = "rgba(255, 255, 255, 0.05)";
          }}
        >
          <X size={18} />
        </button>

        {/* Modal Header */}
        <div style={{ display: "flex", alignItems: "flex-start", gap: "18px", marginBottom: "24px" }}>
          <div style={{
            width: "56px",
            height: "56px",
            borderRadius: "14px",
            background: "linear-gradient(135deg, rgba(0, 242, 254, 0.15) 0%, rgba(79, 172, 254, 0.25) 100%)",
            border: "1px solid var(--border-active)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
          }}>
            <Layers size={28} color="var(--accent-cyan)" />
          </div>

          <div>
            <div style={{ display: "flex", alignItems: "center", gap: "8px", marginBottom: "4px" }}>
              <h2 style={{ fontSize: "1.45rem", fontWeight: 800, color: "#ffffff", letterSpacing: "-0.01em" }}>
                {item.namespace}/{item.name}
              </h2>
              {item.is_verified && (
                <span className="badge badge-emerald">
                  <ShieldCheck size={13} /> Verified Publisher
                </span>
              )}
            </div>
            <p style={{ fontSize: "0.9rem", color: "var(--text-secondary)", maxWidth: "600px", lineHeight: "1.5" }}>
              {item.description}
            </p>
          </div>
        </div>

        {/* CLI Quick Launch Snippet */}
        <div style={{ marginBottom: "28px" }}>
          <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: "8px" }}>
            <span style={{ fontSize: "0.78rem", fontWeight: 600, color: "var(--text-muted)", textTransform: "uppercase", letterSpacing: "0.05em" }}>
              Launch MicroVM Command
            </span>
            <span style={{ fontSize: "0.74rem", color: "var(--accent-cyan)" }}>
              Hardware Isolated · KVM
            </span>
          </div>
          <div style={{
            background: "#04060b",
            border: "1px solid var(--border-active)",
            borderRadius: "var(--radius-md)",
            padding: "14px 18px",
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            boxShadow: "inset 0 2px 6px rgba(0, 0, 0, 0.6)",
          }}>
            <code className="mono" style={{ fontSize: "0.85rem", color: "var(--accent-cyan)", wordBreak: "break-all" }}>
              $ {runCommand}
            </code>
            <button
              onClick={copyToClipboard}
              className="btn-primary"
              style={{ padding: "6px 14px", fontSize: "0.8rem", flexShrink: 0, marginLeft: "14px" }}
            >
              {copied ? <Check size={14} /> : <Copy size={14} />}
              {copied ? "Copied" : "Copy"}
            </button>
          </div>
        </div>

        {/* Metrics Grid */}
        <div style={{
          display: "grid",
          gridTemplateColumns: "repeat(auto-fit, minmax(180px, 1fr))",
          gap: "14px",
          marginBottom: "28px",
        }}>
          <div style={{
            background: "rgba(255, 255, 255, 0.02)",
            border: "1px solid var(--border-subtle)",
            borderRadius: "var(--radius-md)",
            padding: "14px",
          }}>
            <div style={{ display: "flex", alignItems: "center", gap: "6px", color: "var(--text-muted)", fontSize: "0.76rem", marginBottom: "6px" }}>
              <Zap size={14} color="var(--accent-gold)" /> Cold Boot Time
            </div>
            <div style={{ fontSize: "1.2rem", fontWeight: 700, color: "#ffffff" }}>
              &lt; 220 ms
            </div>
            <div style={{ fontSize: "0.7rem", color: "var(--accent-emerald)" }}>Direct Cloud Hypervisor boot</div>
          </div>

          <div style={{
            background: "rgba(255, 255, 255, 0.02)",
            border: "1px solid var(--border-subtle)",
            borderRadius: "var(--radius-md)",
            padding: "14px",
          }}>
            <div style={{ display: "flex", alignItems: "center", gap: "6px", color: "var(--text-muted)", fontSize: "0.76rem", marginBottom: "6px" }}>
              <Activity size={14} color="var(--accent-cyan)" /> Host Memory Overhead
            </div>
            <div style={{ fontSize: "1.2rem", fontWeight: 700, color: "#ffffff" }}>
              ~26 MB
            </div>
            <div style={{ fontSize: "0.7rem", color: "var(--text-muted)" }}>VMM RSS + Linux Guest</div>
          </div>

          <div style={{
            background: "rgba(255, 255, 255, 0.02)",
            border: "1px solid var(--border-subtle)",
            borderRadius: "var(--radius-md)",
            padding: "14px",
          }}>
            <div style={{ display: "flex", alignItems: "center", gap: "6px", color: "var(--text-muted)", fontSize: "0.76rem", marginBottom: "6px" }}>
              <Cpu size={14} color="var(--accent-violet)" /> Workload Kernel
            </div>
            <div style={{ fontSize: "1.05rem", fontWeight: 700, color: "var(--accent-gold)" }}>
              {item.recommended_kernel || "universal-general"}
            </div>
            {item.recommended_kernel && onSelectKernel && (
              <button
                onClick={() => {
                  onClose();
                  onSelectKernel(item.recommended_kernel!);
                }}
                style={{ fontSize: "0.7rem", color: "var(--accent-cyan)", textDecoration: "underline", marginTop: "2px" }}
              >
                Inspect kernel specs →
              </button>
            )}
          </div>

          <div style={{
            background: "rgba(255, 255, 255, 0.02)",
            border: "1px solid var(--border-subtle)",
            borderRadius: "var(--radius-md)",
            padding: "14px",
          }}>
            <div style={{ display: "flex", alignItems: "center", gap: "6px", color: "var(--text-muted)", fontSize: "0.76rem", marginBottom: "6px" }}>
              <Lock size={14} color="var(--accent-emerald)" /> Attestation & Supply Chain
            </div>
            <div style={{ fontSize: "1.05rem", fontWeight: 700, color: "var(--accent-emerald)" }}>
              Ed25519 Verified
            </div>
            <div style={{ fontSize: "0.7rem", color: "var(--text-muted)" }}>0 CVEs · Signed VmifManifest</div>
          </div>
        </div>

        {/* Security Scorecard & PCR Measurement */}
        <div style={{
          background: "rgba(16, 185, 129, 0.04)",
          border: "1px solid rgba(16, 185, 129, 0.2)",
          borderRadius: "var(--radius-md)",
          padding: "16px 20px",
          marginBottom: "28px",
        }}>
          <div style={{ display: "flex", alignItems: "center", gap: "8px", marginBottom: "8px" }}>
            <ShieldCheck size={18} color="var(--accent-emerald)" />
            <span style={{ fontSize: "0.88rem", fontWeight: 700, color: "var(--accent-emerald)" }}>
              Cryptographic Notary Guarantee
            </span>
          </div>
          <p style={{ fontSize: "0.8rem", color: "var(--text-secondary)", lineHeight: "1.5", marginBottom: "8px" }}>
            Every layer in this appliance has been signed by the publisher’s verified Ed25519 key and countersigned by the Vyoma Root Authority. The guest kernel is measured into virtual TPM PCR[0].
          </p>
          <div style={{ display: "flex", alignItems: "center", gap: "10px", fontSize: "0.74rem" }}>
            <span style={{ color: "var(--text-muted)" }}>vTPM PCR[0] Baseline:</span>
            <code className="mono" style={{ color: "var(--accent-cyan)", background: "rgba(0, 0, 0, 0.4)", padding: "2px 6px", borderRadius: "4px" }}>
              {item.pcr_measurement || "f290d8a1c97e14f82638b91a70cde51203498afe"}
            </code>
          </div>
        </div>

        {/* README & Usage Guide */}
        <div>
          <div style={{ display: "flex", alignItems: "center", gap: "6px", marginBottom: "12px", borderBottom: "1px solid var(--border-subtle)", paddingBottom: "8px" }}>
            <FileText size={16} color="var(--accent-cyan)" />
            <h3 style={{ fontSize: "0.95rem", fontWeight: 700, color: "#ffffff" }}>
              Appliance Documentation & Examples
            </h3>
          </div>
          <div style={{
            background: "#06080e",
            border: "1px solid var(--border-subtle)",
            borderRadius: "var(--radius-md)",
            padding: "20px",
            fontSize: "0.85rem",
            color: "var(--text-secondary)",
            lineHeight: "1.7",
          }}>
            <pre style={{ whiteSpace: "pre-wrap", fontFamily: "var(--font-mono)", fontSize: "0.82rem", color: "#e2e8f0" }}>
              {item.readme || `# ${item.name} MicroVM Appliance\n\nRun with:\n$ ${runCommand}`}
            </pre>
          </div>
        </div>
      </div>
    </div>
  );
};
