"use client";

import React from "react";
import {
  ShieldCheck,
  Cpu,
  Layers,
  Key,
  Search,
  Building2,
} from "lucide-react";

export type HubTab = "store" | "kernels" | "notary" | "orgs" | "tokens";

interface NavbarProps {
  activeTab: HubTab;
  onSelectTab: (tab: HubTab) => void;
  searchQuery: string;
  onSearchChange: (q: string) => void;
  isBackendConnected: boolean;
}

export const Navbar: React.FC<NavbarProps> = ({
  activeTab,
  onSelectTab,
  searchQuery,
  onSearchChange,
  isBackendConnected,
}) => {
  return (
    <header style={{
      position: "sticky",
      top: 0,
      zIndex: 100,
      background: "rgba(7, 9, 14, 0.85)",
      backdropFilter: "blur(20px)",
      WebkitBackdropFilter: "blur(20px)",
      borderBottom: "1px solid var(--border-subtle)",
    }}>
      <div style={{
        maxWidth: "1400px",
        margin: "0 auto",
        padding: "0 24px",
        height: "72px",
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        gap: "20px",
      }}>
        {/* Brand / Logo */}
        <div style={{ display: "flex", alignItems: "center", gap: "14px" }}>
          <div style={{
            width: "38px",
            height: "38px",
            borderRadius: "10px",
            background: "linear-gradient(135deg, #00f2fe 0%, #4facfe 100%)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            boxShadow: "0 0 16px rgba(0, 242, 254, 0.4)",
          }}>
            <Layers size={22} color="#040914" strokeWidth={2.5} />
          </div>
          <div>
            <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
              <span style={{ fontSize: "1.15rem", fontWeight: 800, letterSpacing: "-0.02em" }}>
                Vyoma<span style={{ color: "var(--accent-cyan)" }}>Hub</span>
              </span>
              <span className="badge badge-cyan" style={{ fontSize: "0.68rem" }}>
                v0.1.0
              </span>
            </div>
            <div style={{ fontSize: "0.72rem", color: "var(--text-muted)" }}>
              Micro-OS Appliances & Kernel Registry
            </div>
          </div>
        </div>

        {/* Global Search Bar */}
        <div style={{
          flex: 1,
          maxWidth: "420px",
          position: "relative",
          display: "flex",
          alignItems: "center",
        }}>
          <Search size={16} color="var(--text-muted)" style={{ position: "absolute", left: "14px" }} />
          <input
            type="text"
            placeholder="Search appliances, kernels, or tags... (Ctrl+K)"
            value={searchQuery}
            onChange={(e) => onSearchChange(e.target.value)}
            style={{
              width: "100%",
              height: "40px",
              paddingLeft: "40px",
              paddingRight: "14px",
              background: "rgba(19, 24, 38, 0.8)",
              border: "1px solid var(--border-default)",
              borderRadius: "var(--radius-md)",
              fontSize: "0.85rem",
              color: "var(--text-primary)",
              transition: "all 0.2s ease",
            }}
            onFocus={(e) => {
              e.currentTarget.style.borderColor = "var(--accent-cyan)";
              e.currentTarget.style.boxShadow = "0 0 12px rgba(0, 242, 254, 0.2)";
            }}
            onBlur={(e) => {
              e.currentTarget.style.borderColor = "var(--border-default)";
              e.currentTarget.style.boxShadow = "none";
            }}
          />
        </div>

        {/* Nav Tabs */}
        <nav style={{ display: "flex", alignItems: "center", gap: "6px" }}>
          <button
            onClick={() => onSelectTab("store")}
            style={{
              display: "flex",
              alignItems: "center",
              gap: "7px",
              padding: "8px 14px",
              borderRadius: "var(--radius-md)",
              fontSize: "0.88rem",
              fontWeight: 500,
              color: activeTab === "store" ? "#ffffff" : "var(--text-secondary)",
              background: activeTab === "store" ? "rgba(0, 242, 254, 0.12)" : "transparent",
              border: `1px solid ${activeTab === "store" ? "rgba(0, 242, 254, 0.3)" : "transparent"}`,
              transition: "all 0.2s ease",
            }}
          >
            <Layers size={16} color={activeTab === "store" ? "var(--accent-cyan)" : "currentColor"} />
            Appliances
          </button>

          <button
            onClick={() => onSelectTab("kernels")}
            style={{
              display: "flex",
              alignItems: "center",
              gap: "7px",
              padding: "8px 14px",
              borderRadius: "var(--radius-md)",
              fontSize: "0.88rem",
              fontWeight: 500,
              color: activeTab === "kernels" ? "#ffffff" : "var(--text-secondary)",
              background: activeTab === "kernels" ? "rgba(246, 211, 101, 0.12)" : "transparent",
              border: `1px solid ${activeTab === "kernels" ? "rgba(246, 211, 101, 0.3)" : "transparent"}`,
              transition: "all 0.2s ease",
            }}
          >
            <Cpu size={16} color={activeTab === "kernels" ? "var(--accent-gold)" : "currentColor"} />
            Kernels
          </button>

          <button
            onClick={() => onSelectTab("notary")}
            style={{
              display: "flex",
              alignItems: "center",
              gap: "7px",
              padding: "8px 14px",
              borderRadius: "var(--radius-md)",
              fontSize: "0.88rem",
              fontWeight: 500,
              color: activeTab === "notary" ? "#ffffff" : "var(--text-secondary)",
              background: activeTab === "notary" ? "rgba(16, 185, 129, 0.12)" : "transparent",
              border: `1px solid ${activeTab === "notary" ? "rgba(16, 185, 129, 0.3)" : "transparent"}`,
              transition: "all 0.2s ease",
            }}
          >
            <ShieldCheck size={16} color={activeTab === "notary" ? "var(--accent-emerald)" : "currentColor"} />
            Security & Notary
          </button>

          <button
            onClick={() => onSelectTab("orgs")}
            style={{
              display: "flex",
              alignItems: "center",
              gap: "7px",
              padding: "8px 14px",
              borderRadius: "var(--radius-md)",
              fontSize: "0.88rem",
              fontWeight: 500,
              color: activeTab === "orgs" ? "#ffffff" : "var(--text-secondary)",
              background: activeTab === "orgs" ? "rgba(0, 242, 254, 0.12)" : "transparent",
              border: `1px solid ${activeTab === "orgs" ? "rgba(0, 242, 254, 0.3)" : "transparent"}`,
              transition: "all 0.2s ease",
            }}
          >
            <Building2 size={16} color={activeTab === "orgs" ? "var(--accent-cyan)" : "currentColor"} />
            Organizations
          </button>

          <button
            onClick={() => onSelectTab("tokens")}
            style={{
              display: "flex",
              alignItems: "center",
              gap: "7px",
              padding: "8px 14px",
              borderRadius: "var(--radius-md)",
              fontSize: "0.88rem",
              fontWeight: 500,
              color: activeTab === "tokens" ? "#ffffff" : "var(--text-secondary)",
              background: activeTab === "tokens" ? "rgba(139, 92, 246, 0.12)" : "transparent",
              border: `1px solid ${activeTab === "tokens" ? "rgba(139, 92, 246, 0.3)" : "transparent"}`,
              transition: "all 0.2s ease",
            }}
          >
            <Key size={16} color={activeTab === "tokens" ? "var(--accent-violet)" : "currentColor"} />
            CLI Access
          </button>
        </nav>

        {/* Backend Status indicator */}
        <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
          <div
            title={isBackendConnected ? "Connected to Axum backend on port 8081" : "Operating in resilient offline preview mode"}
            style={{
              display: "flex",
              alignItems: "center",
              gap: "6px",
              padding: "5px 11px",
              borderRadius: "9999px",
              background: isBackendConnected ? "rgba(16, 185, 129, 0.1)" : "rgba(246, 211, 101, 0.1)",
              border: `1px solid ${isBackendConnected ? "rgba(16, 185, 129, 0.3)" : "rgba(246, 211, 101, 0.3)"}`,
              fontSize: "0.74rem",
              fontWeight: 600,
              color: isBackendConnected ? "var(--accent-emerald)" : "var(--accent-gold)",
            }}
          >
            <span style={{
              width: "7px",
              height: "7px",
              borderRadius: "50%",
              background: isBackendConnected ? "var(--accent-emerald)" : "var(--accent-gold)",
              boxShadow: isBackendConnected ? "0 0 8px #10b981" : "0 0 8px #f6d365",
            }} />
            {isBackendConnected ? "API Connected (8081)" : "Catalog Active"}
          </div>
        </div>
      </div>
    </header>
  );
};
