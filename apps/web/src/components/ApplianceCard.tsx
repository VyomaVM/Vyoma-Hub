"use client";

import React, { useState } from "react";
import { CatalogItem } from "@/types/hub";
import {
  ShieldCheck,
  Download,
  Star,
  Copy,
  Check,
  Cpu,
  ArrowRight,
  Database,
  Globe,
  Bot,
  Terminal,
} from "lucide-react";

interface ApplianceCardProps {
  item: CatalogItem;
  onOpenDetails: (item: CatalogItem) => void;
  onSelectKernel?: (kernelName: string) => void;
}

export const ApplianceCard: React.FC<ApplianceCardProps> = ({
  item,
  onOpenDetails,
  onSelectKernel,
}) => {
  const [copied, setCopied] = useState(false);

  const runCommand = `vyoma run hub.vyoma.dev/${item.namespace}/${item.name}:${item.tags[0] || "latest"}`;

  const copyToClipboard = (e: React.MouseEvent) => {
    e.stopPropagation();
    navigator.clipboard.writeText(runCommand);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const getCategoryIcon = () => {
    switch (item.category) {
      case "database":
        return <Database size={18} color="var(--accent-gold)" />;
      case "network":
        return <Globe size={18} color="var(--accent-cyan)" />;
      case "ai":
        return <Bot size={18} color="var(--accent-violet)" />;
      default:
        return <Terminal size={18} color="var(--accent-emerald)" />;
    }
  };

  const formatNumber = (num: number) => {
    if (num >= 1000) return `${(num / 1000).toFixed(1)}k`;
    return num.toString();
  };

  return (
    <div
      onClick={() => onOpenDetails(item)}
      className="glass-panel"
      style={{
        padding: "24px",
        display: "flex",
        flexDirection: "column",
        justifyContent: "space-between",
        cursor: "pointer",
        position: "relative",
        overflow: "hidden",
      }}
    >
      {/* Top Header */}
      <div>
        <div style={{ display: "flex", alignItems: "flex-start", justifyContent: "space-between", marginBottom: "14px" }}>
          <div style={{ display: "flex", alignItems: "center", gap: "12px" }}>
            <div style={{
              width: "42px",
              height: "42px",
              borderRadius: "10px",
              background: "rgba(255, 255, 255, 0.04)",
              border: "1px solid var(--border-default)",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
            }}>
              {getCategoryIcon()}
            </div>
            <div>
              <div style={{ display: "flex", alignItems: "center", gap: "6px" }}>
                <span style={{ fontSize: "1.05rem", fontWeight: 700, color: "#ffffff" }}>
                  {item.namespace}/{item.name}
                </span>
                {item.is_verified && (
                  <span title="Verified Publisher by Vyoma Notary" style={{ display: "inline-flex" }}>
                    <ShieldCheck size={16} color="var(--accent-emerald)" />
                  </span>
                )}
              </div>
              <div style={{ fontSize: "0.75rem", color: "var(--text-muted)" }}>
                Official Hardware-Isolated Micro-OS
              </div>
            </div>
          </div>

          {/* Recommended Kernel Badge */}
          {item.recommended_kernel && (
            <button
              onClick={(e) => {
                e.stopPropagation();
                if (onSelectKernel) onSelectKernel(item.recommended_kernel!);
              }}
              className="badge badge-gold"
              title="Click to view recommended kernel details"
              style={{
                display: "flex",
                alignItems: "center",
                gap: "4px",
                cursor: "pointer",
              }}
            >
              <Cpu size={12} />
              {item.recommended_kernel}
            </button>
          )}
        </div>

        {/* Description */}
        <p style={{
          fontSize: "0.85rem",
          color: "var(--text-secondary)",
          lineHeight: "1.5",
          marginBottom: "18px",
          minHeight: "40px",
        }}>
          {item.description}
        </p>
      </div>

      {/* Middle Tags & Specs */}
      <div>
        <div style={{ display: "flex", flexWrap: "wrap", gap: "6px", marginBottom: "16px" }}>
          {item.tags.slice(0, 3).map((tag) => (
            <span
              key={tag}
              style={{
                fontSize: "0.72rem",
                padding: "2px 8px",
                borderRadius: "4px",
                background: "rgba(255, 255, 255, 0.04)",
                color: "var(--text-muted)",
                border: "1px solid var(--border-subtle)",
              }}
            >
              {tag}
            </span>
          ))}
          {item.vcpu && (
            <span style={{
              fontSize: "0.72rem",
              padding: "2px 8px",
              borderRadius: "4px",
              background: "rgba(0, 242, 254, 0.08)",
              color: "var(--accent-cyan)",
              border: "1px solid rgba(0, 242, 254, 0.2)",
            }}>
              {item.vcpu} vCPU · {item.memory_mb}MB
            </span>
          )}
        </div>

        {/* One-click CLI command snippet */}
        <div
          onClick={copyToClipboard}
          style={{
            background: "#04060a",
            border: "1px solid var(--border-subtle)",
            borderRadius: "var(--radius-md)",
            padding: "8px 12px",
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            marginBottom: "14px",
            transition: "all 0.2s ease",
          }}
          onMouseEnter={(e) => (e.currentTarget.style.borderColor = "var(--accent-cyan)")}
          onMouseLeave={(e) => (e.currentTarget.style.borderColor = "var(--border-subtle)")}
        >
          <span className="mono" style={{ fontSize: "0.76rem", color: "var(--accent-cyan)", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
            $ {runCommand}
          </span>
          <button
            title="Copy command"
            style={{
              color: copied ? "var(--accent-emerald)" : "var(--text-muted)",
              display: "flex",
              alignItems: "center",
              marginLeft: "8px",
            }}
          >
            {copied ? <Check size={14} /> : <Copy size={14} />}
          </button>
        </div>

        {/* Card Footer */}
        <div style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          paddingTop: "12px",
          borderTop: "1px solid var(--border-subtle)",
          fontSize: "0.78rem",
          color: "var(--text-muted)",
        }}>
          <div style={{ display: "flex", alignItems: "center", gap: "14px" }}>
            <span style={{ display: "flex", alignItems: "center", gap: "4px" }}>
              <Download size={13} />
              {formatNumber(item.pull_count)}
            </span>
            <span style={{ display: "flex", alignItems: "center", gap: "4px" }}>
              <Star size={13} color="var(--accent-gold)" />
              {formatNumber(item.star_count)}
            </span>
          </div>

          <span style={{
            display: "flex",
            alignItems: "center",
            gap: "4px",
            color: "var(--accent-cyan)",
            fontWeight: 600,
          }}>
            Explore <ArrowRight size={13} />
          </span>
        </div>
      </div>
    </div>
  );
};
