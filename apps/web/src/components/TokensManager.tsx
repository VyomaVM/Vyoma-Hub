"use client";

import React, { useState } from "react";
import { UserToken } from "@/types/hub";
import {
  Key,
  Shield,
  Copy,
  Check,
  Terminal,
  Plus,
  Trash2,
} from "lucide-react";

export const TokensManager: React.FC = () => {
  const [tokens, setTokens] = useState<UserToken[]>([
    {
      id: "tok-ci-001",
      name: "GitHub Actions CI/CD Release",
      token_preview: "vyoma_pat_9a8b...12ef",
      scopes: ["repo:pull", "repo:push"],
      created_at: "2026-10-20T10:00:00Z",
      last_used_at: "12 minutes ago",
    },
    {
      id: "tok-dev-002",
      name: "Subesh Workstation CLI",
      token_preview: "vyoma_pat_4c5d...7890",
      scopes: ["repo:pull", "kernel:download", "admin:org"],
      created_at: "2026-10-20T11:30:00Z",
      last_used_at: "Just now",
    },
  ]);

  const [newTokenName, setNewTokenName] = useState("");
  const [selectedScopes, setSelectedScopes] = useState<string[]>([
    "repo:pull",
    "kernel:download",
  ]);
  const [createdToken, setCreatedToken] = useState<string | null>(null);
  const [copiedToken, setCopiedToken] = useState(false);
  const [copiedCli, setCopiedCli] = useState(false);

  const availableScopes = [
    { id: "repo:pull", label: "Pull Appliances & Layers (`repo:pull`)", desc: "Allows pulling public and private VMIF images" },
    { id: "repo:push", label: "Publish & Push Images (`repo:push`)", desc: "Upload new appliances, tags, and manifests" },
    { id: "kernel:download", label: "Download Certified Kernels (`kernel:download`)", desc: "Access guest kernel binaries and configs" },
    { id: "admin:org", label: "Organization Administration (`admin:org`)", desc: "Manage team members, webhooks, and billing" },
  ];

  const handleToggleScope = (scopeId: string) => {
    setSelectedScopes((prev) =>
      prev.includes(scopeId) ? prev.filter((s) => s !== scopeId) : [...prev, scopeId]
    );
  };

  const handleGenerateToken = (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTokenName.trim()) return;

    const randomSuffix = Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
    const fullToken = `vyoma_pat_${randomSuffix}`;

    const newTok: UserToken = {
      id: `tok-${Date.now()}`,
      name: newTokenName.trim(),
      token_preview: `${fullToken.slice(0, 14)}...${fullToken.slice(-4)}`,
      scopes: [...selectedScopes],
      created_at: new Date().toISOString(),
      last_used_at: "Never",
    };

    setTokens([newTok, ...tokens]);
    setCreatedToken(fullToken);
    setNewTokenName("");
  };

  const loginCmd = `vyoma login hub.vyoma.dev -u subesh -p ${createdToken || "vyoma_pat_your_token_here"}`;

  return (
    <div style={{ maxWidth: "1200px", margin: "0 auto" }}>
      {/* Header */}
      <div style={{ marginBottom: "36px" }}>
        <div style={{ display: "inline-flex", alignItems: "center", gap: "6px", marginBottom: "12px" }}>
          <span className="badge badge-violet">
            <Key size={13} /> Personal Access Tokens (PAT)
          </span>
          <span className="badge badge-cyan">CLI & Automation</span>
        </div>
        <h1 style={{ fontSize: "2.2rem", fontWeight: 800, color: "#ffffff", letterSpacing: "-0.02em", marginBottom: "12px" }}>
          CLI Authentication & Scoped Tokens
        </h1>
        <p style={{ fontSize: "1.05rem", color: "var(--text-secondary)", maxWidth: "800px", lineHeight: "1.6" }}>
          Authenticate your terminal or CI/CD runner to pull hardware-isolated appliances, stream OCI layers, and push signed VMIF manifests directly to Vyoma Hub.
        </p>
      </div>

      {/* CLI Quickstart Card */}
      <div className="glass-panel" style={{ padding: "28px", background: "#0c101a", border: "1px solid var(--border-default)", marginBottom: "36px" }}>
        <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "14px" }}>
          <Terminal size={20} color="var(--accent-cyan)" />
          <h3 style={{ fontSize: "1.15rem", fontWeight: 700, color: "#ffffff" }}>
            Terminal Login Command
          </h3>
        </div>
        <div style={{
          background: "#04060a",
          border: "1px solid var(--border-active)",
          borderRadius: "var(--radius-md)",
          padding: "14px 18px",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          boxShadow: "inset 0 2px 6px rgba(0, 0, 0, 0.6)",
        }}>
          <code className="mono" style={{ fontSize: "0.85rem", color: "var(--accent-cyan)", wordBreak: "break-all" }}>
            $ {loginCmd}
          </code>
          <button
            onClick={() => {
              navigator.clipboard.writeText(loginCmd);
              setCopiedCli(true);
              setTimeout(() => setCopiedCli(false), 2000);
            }}
            className="btn-primary"
            style={{ padding: "6px 14px", fontSize: "0.8rem", flexShrink: 0, marginLeft: "14px" }}
          >
            {copiedCli ? <Check size={14} /> : <Copy size={14} />}
            {copiedCli ? "Copied" : "Copy"}
          </button>
        </div>
      </div>

      {/* Generated Token Alert */}
      {createdToken && (
        <div style={{
          background: "rgba(16, 185, 129, 0.08)",
          border: "1px solid rgba(16, 185, 129, 0.4)",
          borderRadius: "var(--radius-lg)",
          padding: "24px",
          marginBottom: "36px",
          position: "relative",
        }}>
          <div style={{ display: "flex", alignItems: "center", gap: "8px", color: "var(--accent-emerald)", fontWeight: 700, marginBottom: "8px" }}>
            <Check size={18} /> New Token Generated Successfully
          </div>
          <p style={{ fontSize: "0.85rem", color: "var(--text-secondary)", marginBottom: "14px" }}>
            Please copy this token immediately. For your security, it will not be displayed again.
          </p>
          <div style={{
            background: "#030509",
            border: "1px solid var(--border-default)",
            borderRadius: "var(--radius-md)",
            padding: "12px 16px",
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
          }}>
            <code className="mono" style={{ fontSize: "0.9rem", color: "var(--accent-emerald)", wordBreak: "break-all" }}>
              {createdToken}
            </code>
            <button
              onClick={() => {
                navigator.clipboard.writeText(createdToken);
                setCopiedToken(true);
                setTimeout(() => setCopiedToken(false), 2000);
              }}
              className="btn-secondary"
              style={{ padding: "6px 12px", fontSize: "0.78rem", flexShrink: 0, marginLeft: "12px" }}
            >
              {copiedToken ? <Check size={14} /> : <Copy size={14} />}
              {copiedToken ? "Copied" : "Copy Token"}
            </button>
          </div>
        </div>
      )}

      {/* Main Grid: Generator & Token List */}
      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "32px" }}>
        {/* Generate Token Form */}
        <div className="glass-panel" style={{ padding: "28px" }}>
          <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "16px" }}>
            <Plus size={20} color="var(--accent-cyan)" />
            <h3 style={{ fontSize: "1.2rem", fontWeight: 700, color: "#ffffff" }}>
              Generate New Token
            </h3>
          </div>

          <form onSubmit={handleGenerateToken}>
            <div style={{ marginBottom: "20px" }}>
              <label style={{ display: "block", fontSize: "0.85rem", fontWeight: 600, color: "var(--text-secondary)", marginBottom: "8px" }}>
                Token Description
              </label>
              <input
                type="text"
                value={newTokenName}
                onChange={(e) => setNewTokenName(e.target.value)}
                placeholder="e.g. Production Cluster Runner"
                required
                style={{
                  width: "100%",
                  padding: "10px 14px",
                  background: "#04060a",
                  border: "1px solid var(--border-default)",
                  borderRadius: "var(--radius-md)",
                  color: "#ffffff",
                  fontSize: "0.88rem",
                }}
              />
            </div>

            <div style={{ marginBottom: "24px" }}>
              <label style={{ display: "block", fontSize: "0.85rem", fontWeight: 600, color: "var(--text-secondary)", marginBottom: "12px" }}>
                Select Scopes & Permissions
              </label>
              <div style={{ display: "flex", flexDirection: "column", gap: "10px" }}>
                {availableScopes.map((scope) => {
                  const isChecked = selectedScopes.includes(scope.id);
                  return (
                    <div
                      key={scope.id}
                      onClick={() => handleToggleScope(scope.id)}
                      style={{
                        padding: "12px 14px",
                        borderRadius: "var(--radius-md)",
                        background: isChecked ? "rgba(0, 242, 254, 0.06)" : "rgba(255, 255, 255, 0.02)",
                        border: `1px solid ${isChecked ? "var(--accent-cyan)" : "var(--border-subtle)"}`,
                        cursor: "pointer",
                        display: "flex",
                        alignItems: "flex-start",
                        gap: "10px",
                        transition: "all 0.15s ease",
                      }}
                    >
                      <input
                        type="checkbox"
                        checked={isChecked}
                        onChange={() => {}}
                        style={{ accentColor: "var(--accent-cyan)", marginTop: "3px" }}
                      />
                      <div>
                        <div style={{ fontSize: "0.85rem", fontWeight: 600, color: isChecked ? "#ffffff" : "var(--text-secondary)" }}>
                          {scope.label}
                        </div>
                        <div style={{ fontSize: "0.74rem", color: "var(--text-muted)", marginTop: "2px" }}>
                          {scope.desc}
                        </div>
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>

            <button type="submit" className="btn-primary" style={{ width: "100%", padding: "12px" }}>
              <Key size={16} /> Generate Access Token
            </button>
          </form>
        </div>

        {/* Existing Tokens List */}
        <div className="glass-panel" style={{ padding: "28px" }}>
          <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: "20px" }}>
            <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
              <Shield size={20} color="var(--accent-gold)" />
              <h3 style={{ fontSize: "1.2rem", fontWeight: 700, color: "#ffffff" }}>
                Active Access Tokens
              </h3>
            </div>
            <span className="badge badge-gold">{tokens.length} Active</span>
          </div>

          <div style={{ display: "flex", flexDirection: "column", gap: "14px" }}>
            {tokens.map((tok) => (
              <div
                key={tok.id}
                style={{
                  background: "rgba(255, 255, 255, 0.02)",
                  border: "1px solid var(--border-subtle)",
                  borderRadius: "var(--radius-md)",
                  padding: "16px",
                }}
              >
                <div style={{ display: "flex", alignItems: "flex-start", justifyContent: "space-between", marginBottom: "8px" }}>
                  <div>
                    <div style={{ fontSize: "0.95rem", fontWeight: 700, color: "#ffffff" }}>
                      {tok.name}
                    </div>
                    <code className="mono" style={{ fontSize: "0.78rem", color: "var(--accent-cyan)", marginTop: "2px", display: "inline-block" }}>
                      {tok.token_preview}
                    </code>
                  </div>

                  <button
                    onClick={() => setTokens(tokens.filter((t) => t.id !== tok.id))}
                    title="Revoke Token"
                    style={{ color: "var(--text-muted)", padding: "4px" }}
                    onMouseEnter={(e) => (e.currentTarget.style.color = "var(--accent-rose)")}
                    onMouseLeave={(e) => (e.currentTarget.style.color = "var(--text-muted)")}
                  >
                    <Trash2 size={16} />
                  </button>
                </div>

                <div style={{ display: "flex", flexWrap: "wrap", gap: "6px", marginBottom: "10px" }}>
                  {tok.scopes.map((s) => (
                    <span
                      key={s}
                      style={{
                        fontSize: "0.7rem",
                        padding: "2px 6px",
                        borderRadius: "4px",
                        background: "rgba(255, 255, 255, 0.04)",
                        color: "var(--text-muted)",
                        border: "1px solid var(--border-subtle)",
                      }}
                    >
                      {s}
                    </span>
                  ))}
                </div>

                <div style={{ fontSize: "0.72rem", color: "var(--text-dim)" }}>
                  Last used: {tok.last_used_at || "Never"}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};
