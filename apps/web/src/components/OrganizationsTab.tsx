"use client";

import React, { useState, useEffect } from "react";
import { Organization, OrgMember, Webhook, WebhookDelivery } from "@/types/hub";
import {
  getOrganizations,
  getOrgMembers,
  addOrgMember,
  getWebhooks,
  createWebhook,
  testWebhook,
  getWebhookDeliveries,
  FALLBACK_ORGS,
} from "@/lib/api";
import {
  Building2,
  Users,
  Webhook as WebhookIcon,
  ShieldCheck,
  Plus,
  Send,
  Radio,
} from "lucide-react";

export const OrganizationsTab: React.FC = () => {
  const [orgs, setOrgs] = useState<Organization[]>(FALLBACK_ORGS);
  const [selectedOrg, setSelectedOrg] = useState<Organization>(FALLBACK_ORGS[0]);
  const [activeSubTab, setActiveSubTab] = useState<"members" | "webhooks" | "settings">("members");

  const [members, setMembers] = useState<OrgMember[]>([]);
  const [webhooks, setWebhooks] = useState<Webhook[]>([]);
  const [deliveries, setDeliveries] = useState<WebhookDelivery[]>([]);
  const [selectedWebhookId, setSelectedWebhookId] = useState<string | null>(null);

  const [dispatching, setDispatching] = useState(false);

  // Invite member modal state
  const [showInviteModal, setShowInviteModal] = useState(false);
  const [inviteUsername, setInviteUsername] = useState("");
  const [inviteRole, setInviteRole] = useState("member");

  // New webhook modal state
  const [showWebhookModal, setShowWebhookModal] = useState(false);
  const [webhookUrl, setWebhookUrl] = useState("");
  const [webhookSecret, setWebhookSecret] = useState("");
  const [webhookEvents, setWebhookEvents] = useState<string[]>(["push", "release", "scan"]);

  // Load organizations
  useEffect(() => {
    async function initOrgs() {
      const res = await getOrganizations();
      setOrgs(res.orgs);
      if (res.orgs.length > 0) {
        setSelectedOrg(res.orgs[0]);
      }
    }
    initOrgs();
  }, []);

  // Load members and webhooks when selectedOrg changes
  useEffect(() => {
    if (!selectedOrg) return;

    async function loadOrgData() {
      const [mems, hooks] = await Promise.all([
        getOrgMembers(selectedOrg.name),
        getWebhooks(selectedOrg.name),
      ]);
      setMembers(mems);
      setWebhooks(hooks);

      if (hooks.length > 0) {
        setSelectedWebhookId(hooks[0].id);
        const dels = await getWebhookDeliveries(selectedOrg.name, hooks[0].id);
        setDeliveries(dels);
      } else {
        setSelectedWebhookId(null);
        setDeliveries([]);
      }
    }

    loadOrgData();
  }, [selectedOrg]);

  // Handle Webhook selection
  const handleSelectWebhook = async (id: string) => {
    setSelectedWebhookId(id);
    const dels = await getWebhookDeliveries(selectedOrg.name, id);
    setDeliveries(dels);
  };

  // Handle Add Member
  const handleInviteSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!inviteUsername.trim()) return;

    const newMember = await addOrgMember(selectedOrg.name, {
      username: inviteUsername.trim(),
      role: inviteRole,
    });

    if (newMember) {
      setMembers((prev) => [...prev.filter((m) => m.username !== newMember.username), newMember]);
    }
    setInviteUsername("");
    setShowInviteModal(false);
  };

  // Handle Create Webhook
  const handleWebhookSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!webhookUrl.trim()) return;

    const newHook = await createWebhook(selectedOrg.name, {
      url: webhookUrl.trim(),
      secret: webhookSecret.trim() || undefined,
      events: webhookEvents,
    });

    if (newHook) {
      setWebhooks((prev) => [newHook, ...prev]);
      setSelectedWebhookId(newHook.id);
    }
    setWebhookUrl("");
    setWebhookSecret("");
    setShowWebhookModal(false);
  };

  // Handle Test Dispatch
  const handleTestDispatch = async () => {
    if (!selectedWebhookId) return;
    setDispatching(true);

    const delivery = await testWebhook(selectedOrg.name, selectedWebhookId);
    setDispatching(false);

    if (delivery) {
      setDeliveries((prev) => [delivery, ...prev]);
    }
  };

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: "32px" }}>
      {/* Enterprise Header Banner */}
      <div
        className="card"
        style={{
          padding: "32px",
          background: "linear-gradient(135deg, rgba(16, 22, 36, 0.95) 0%, rgba(8, 12, 22, 0.95) 100%)",
          border: "1px solid rgba(0, 242, 254, 0.2)",
          position: "relative",
          overflow: "hidden",
        }}
      >
        <div style={{ display: "flex", justifyContent: "space-between", alignItems: "flex-start", flexWrap: "wrap", gap: "20px" }}>
          <div>
            <div style={{ display: "inline-flex", alignItems: "center", gap: "8px", marginBottom: "12px" }}>
              <span className="badge badge-cyan">
                <Building2 size={13} /> Enterprise Multi-Tenancy
              </span>
              <span className="badge badge-gold">
                <ShieldCheck size={13} /> Fine-Grained RBAC
              </span>
              <span className="badge badge-emerald">
                <Radio size={13} /> HMAC-SHA256 Webhooks
              </span>
            </div>
            <h1 style={{ fontSize: "2rem", fontWeight: 700, margin: "0 0 10px 0", color: "#fff" }}>
              Organization Command Center
            </h1>
            <p style={{ color: "var(--text-secondary)", margin: 0, maxWidth: "700px", fontSize: "0.95rem", lineHeight: 1.6 }}>
              Manage team access tiers, multi-tenant guest namespace isolation, and signed event dispatching for automated continuous integration.
            </p>
          </div>

          {/* Quick Stats Pills */}
          <div style={{ display: "flex", gap: "12px", flexWrap: "wrap" }}>
            <div style={{
              background: "rgba(0, 242, 254, 0.05)",
              border: "1px solid rgba(0, 242, 254, 0.2)",
              borderRadius: "12px",
              padding: "12px 18px",
              textAlign: "center",
            }}>
              <div style={{ fontSize: "1.4rem", fontWeight: 700, color: "var(--neon-cyan)" }}>
                {members.length}
              </div>
              <div style={{ fontSize: "0.75rem", color: "var(--text-muted)", textTransform: "uppercase", letterSpacing: "0.5px" }}>
                Active Members
              </div>
            </div>

            <div style={{
              background: "rgba(16, 185, 129, 0.05)",
              border: "1px solid rgba(16, 185, 129, 0.2)",
              borderRadius: "12px",
              padding: "12px 18px",
              textAlign: "center",
            }}>
              <div style={{ fontSize: "1.4rem", fontWeight: 700, color: "#10b981" }}>
                {webhooks.length}
              </div>
              <div style={{ fontSize: "0.75rem", color: "var(--text-muted)", textTransform: "uppercase", letterSpacing: "0.5px" }}>
                Live Webhooks
              </div>
            </div>

            <div style={{
              background: "rgba(245, 158, 11, 0.05)",
              border: "1px solid rgba(245, 158, 11, 0.2)",
              borderRadius: "12px",
              padding: "12px 18px",
              textAlign: "center",
            }}>
              <div style={{ fontSize: "1.4rem", fontWeight: 700, color: "#f59e0b" }}>
                100%
              </div>
              <div style={{ fontSize: "0.75rem", color: "var(--text-muted)", textTransform: "uppercase", letterSpacing: "0.5px" }}>
                Delivery Health
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Main Multi-Tenant Workspace Layout */}
      <div style={{ display: "grid", gridTemplateColumns: "300px 1fr", gap: "24px", alignItems: "start" }}>
        {/* Left Column: Organization Switcher */}
        <div className="card" style={{ padding: "20px" }}>
          <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "16px" }}>
            <span style={{ fontSize: "0.85rem", fontWeight: 600, color: "var(--text-muted)", textTransform: "uppercase", letterSpacing: "0.5px" }}>
              Organizations
            </span>
            <span className="badge badge-purple" style={{ fontSize: "0.7rem" }}>
              {orgs.length} Total
            </span>
          </div>

          <div style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
            {orgs.map((org) => {
              const isSelected = selectedOrg?.name === org.name;
              return (
                <button
                  key={org.id}
                  onClick={() => setSelectedOrg(org)}
                  style={{
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "space-between",
                    padding: "12px 14px",
                    borderRadius: "10px",
                    border: isSelected
                      ? "1px solid var(--neon-cyan)"
                      : "1px solid var(--border-subtle)",
                    background: isSelected
                      ? "rgba(0, 242, 254, 0.08)"
                      : "rgba(255, 255, 255, 0.02)",
                    color: isSelected ? "#fff" : "var(--text-secondary)",
                    cursor: "pointer",
                    textAlign: "left",
                    transition: "all 0.2s ease",
                  }}
                >
                  <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
                    <div style={{
                      width: "32px",
                      height: "32px",
                      borderRadius: "8px",
                      background: isSelected
                        ? "linear-gradient(135deg, var(--neon-cyan) 0%, #4facfe 100%)"
                        : "rgba(255, 255, 255, 0.08)",
                      color: isSelected ? "#07090e" : "#fff",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                      fontWeight: 700,
                      fontSize: "0.9rem",
                    }}>
                      {org.display_name.charAt(0)}
                    </div>
                    <div>
                      <div style={{ fontWeight: 600, fontSize: "0.9rem", color: "#fff" }}>
                        {org.display_name}
                      </div>
                      <div style={{ fontSize: "0.75rem", color: "var(--text-muted)" }}>
                        @{org.name}
                      </div>
                    </div>
                  </div>

                  {org.is_verified && (
                    <ShieldCheck size={16} color="var(--neon-cyan)" />
                  )}
                </button>
              );
            })}
          </div>

          <div style={{ marginTop: "24px", paddingTop: "16px", borderTop: "1px solid var(--border-subtle)" }}>
            <div style={{
              padding: "12px",
              background: "rgba(0, 242, 254, 0.03)",
              border: "1px dashed rgba(0, 242, 254, 0.2)",
              borderRadius: "8px",
              fontSize: "0.8rem",
              color: "var(--text-muted)",
              lineHeight: 1.5,
            }}>
              <div style={{ color: "var(--neon-cyan)", fontWeight: 600, marginBottom: "4px" }}>
                Multi-Tenant Isolation
              </div>
              Repositories published under <code>{"vyoma/*"}</code> are cryptographically separated from customer namespaces.
            </div>
          </div>
        </div>

        {/* Right Column: Organization Details & Sub-Tabs */}
        <div style={{ display: "flex", flexDirection: "column", gap: "20px" }}>
          {/* Sub-Tab Navigation */}
          <div style={{
            display: "flex",
            alignItems: "center",
            gap: "8px",
            borderBottom: "1px solid var(--border-subtle)",
            paddingBottom: "12px",
          }}>
            <button
              onClick={() => setActiveSubTab("members")}
              style={{
                display: "inline-flex",
                alignItems: "center",
                gap: "8px",
                padding: "8px 16px",
                borderRadius: "8px",
                border: "none",
                background: activeSubTab === "members" ? "rgba(0, 242, 254, 0.12)" : "transparent",
                color: activeSubTab === "members" ? "var(--neon-cyan)" : "var(--text-secondary)",
                fontWeight: 600,
                fontSize: "0.9rem",
                cursor: "pointer",
              }}
            >
              <Users size={16} /> Team & RBAC ({members.length})
            </button>

            <button
              onClick={() => setActiveSubTab("webhooks")}
              style={{
                display: "inline-flex",
                alignItems: "center",
                gap: "8px",
                padding: "8px 16px",
                borderRadius: "8px",
                border: "none",
                background: activeSubTab === "webhooks" ? "rgba(0, 242, 254, 0.12)" : "transparent",
                color: activeSubTab === "webhooks" ? "var(--neon-cyan)" : "var(--text-secondary)",
                fontWeight: 600,
                fontSize: "0.9rem",
                cursor: "pointer",
              }}
            >
              <WebhookIcon size={16} /> Signed Webhooks ({webhooks.length})
            </button>

            <button
              onClick={() => setActiveSubTab("settings")}
              style={{
                display: "inline-flex",
                alignItems: "center",
                gap: "8px",
                padding: "8px 16px",
                borderRadius: "8px",
                border: "none",
                background: activeSubTab === "settings" ? "rgba(0, 242, 254, 0.12)" : "transparent",
                color: activeSubTab === "settings" ? "var(--neon-cyan)" : "var(--text-secondary)",
                fontWeight: 600,
                fontSize: "0.9rem",
                cursor: "pointer",
              }}
            >
              <ShieldCheck size={16} /> Trust & Policies
            </button>
          </div>

          {/* SUBTAB 1: MEMBERS & RBAC */}
          {activeSubTab === "members" && (
            <div className="card" style={{ padding: "24px" }}>
              <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "20px" }}>
                <div>
                  <h3 style={{ margin: "0 0 4px 0", fontSize: "1.2rem", color: "#fff" }}>
                    Team Membership & Permissions
                  </h3>
                  <p style={{ margin: 0, color: "var(--text-muted)", fontSize: "0.85rem" }}>
                    Configure role-based access for appliances published under <code>{`${selectedOrg?.name}/*`}</code>
                  </p>
                </div>

                <button
                  onClick={() => setShowInviteModal(true)}
                  className="btn btn-primary"
                  style={{ display: "inline-flex", alignItems: "center", gap: "6px", fontSize: "0.85rem" }}
                >
                  <Plus size={15} /> Invite Member
                </button>
              </div>

              {/* Members Table */}
              <div style={{ overflowX: "auto" }}>
                <table style={{ width: "100%", borderCollapse: "collapse", textAlign: "left", fontSize: "0.9rem" }}>
                  <thead>
                    <tr style={{ borderBottom: "1px solid var(--border-subtle)", color: "var(--text-muted)" }}>
                      <th style={{ padding: "12px 16px" }}>User</th>
                      <th style={{ padding: "12px 16px" }}>Role</th>
                      <th style={{ padding: "12px 16px" }}>Capabilities</th>
                      <th style={{ padding: "12px 16px" }}>Joined</th>
                    </tr>
                  </thead>
                  <tbody>
                    {members.map((member) => {
                      const roleBadge =
                        member.role === "owner" ? (
                          <span className="badge badge-gold">Owner</span>
                        ) : member.role === "admin" ? (
                          <span className="badge badge-purple">Admin</span>
                        ) : (
                          <span className="badge badge-cyan">Member</span>
                        );

                      const capabilities =
                        member.role === "owner"
                          ? "Full Org Admin, Billing, Key Signing, Member Management"
                          : member.role === "admin"
                          ? "Manage Repositories, Configure Webhooks, Invite Members"
                          : "Pull Private Images, Push Appliance Releases";

                      return (
                        <tr
                          key={member.user_id}
                          style={{
                            borderBottom: "1px solid rgba(255, 255, 255, 0.04)",
                            transition: "background 0.2s",
                          }}
                        >
                          <td style={{ padding: "14px 16px" }}>
                            <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
                              <div style={{
                                width: "30px",
                                height: "30px",
                                borderRadius: "50%",
                                background: "rgba(0, 242, 254, 0.15)",
                                color: "var(--neon-cyan)",
                                display: "flex",
                                alignItems: "center",
                                justifyContent: "center",
                                fontWeight: 700,
                                fontSize: "0.8rem",
                              }}>
                                {member.username.charAt(0).toUpperCase()}
                              </div>
                              <div>
                                <div style={{ fontWeight: 600, color: "#fff" }}>
                                  {member.full_name || member.username}
                                </div>
                                <div style={{ fontSize: "0.75rem", color: "var(--text-muted)" }}>
                                  @{member.username}
                                </div>
                              </div>
                            </div>
                          </td>
                          <td style={{ padding: "14px 16px" }}>{roleBadge}</td>
                          <td style={{ padding: "14px 16px", color: "var(--text-secondary)", fontSize: "0.8rem" }}>
                            {capabilities}
                          </td>
                          <td style={{ padding: "14px 16px", color: "var(--text-muted)", fontSize: "0.8rem" }}>
                            {new Date(member.joined_at).toLocaleDateString()}
                          </td>
                        </tr>
                      );
                    })}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {/* SUBTAB 2: SIGNED WEBHOOKS */}
          {activeSubTab === "webhooks" && (
            <div style={{ display: "flex", flexDirection: "column", gap: "20px" }}>
              <div className="card" style={{ padding: "24px" }}>
                <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "20px" }}>
                  <div>
                    <h3 style={{ margin: "0 0 4px 0", fontSize: "1.2rem", color: "#fff" }}>
                      Cryptographic Webhook Endpoints
                    </h3>
                    <p style={{ margin: 0, color: "var(--text-muted)", fontSize: "0.85rem" }}>
                      Dispatches HMAC-SHA256 signed JSON payloads for CI/CD pipeline triggers and release notifications.
                    </p>
                  </div>

                  <button
                    onClick={() => setShowWebhookModal(true)}
                    className="btn btn-primary"
                    style={{ display: "inline-flex", alignItems: "center", gap: "6px", fontSize: "0.85rem" }}
                  >
                    <Plus size={15} /> Add Webhook
                  </button>
                </div>

                {webhooks.length === 0 ? (
                  <div style={{ padding: "40px", textAlign: "center", color: "var(--text-muted)" }}>
                    No webhooks configured yet for @{selectedOrg?.name}.
                  </div>
                ) : (
                  <div style={{ display: "flex", flexDirection: "column", gap: "12px" }}>
                    {webhooks.map((hook) => {
                      const isSelected = selectedWebhookId === hook.id;
                      return (
                        <div
                          key={hook.id}
                          style={{
                            padding: "16px",
                            borderRadius: "10px",
                            border: isSelected
                              ? "1px solid var(--neon-cyan)"
                              : "1px solid var(--border-subtle)",
                            background: isSelected
                              ? "rgba(0, 242, 254, 0.04)"
                              : "rgba(255, 255, 255, 0.01)",
                            display: "flex",
                            justifyContent: "space-between",
                            alignItems: "center",
                            flexWrap: "wrap",
                            gap: "12px",
                          }}
                        >
                          <div>
                            <div style={{ display: "flex", alignItems: "center", gap: "8px", marginBottom: "6px" }}>
                              <span style={{ fontWeight: 600, color: "#fff", fontFamily: "var(--font-mono)", fontSize: "0.95rem" }}>
                                {hook.url}
                              </span>
                              <span className="badge badge-emerald" style={{ fontSize: "0.7rem" }}>
                                Active
                              </span>
                            </div>
                            <div style={{ display: "flex", alignItems: "center", gap: "6px" }}>
                              {hook.events.map((ev) => (
                                <span
                                  key={ev}
                                  style={{
                                    fontSize: "0.75rem",
                                    padding: "2px 8px",
                                    background: "rgba(255, 255, 255, 0.06)",
                                    borderRadius: "4px",
                                    color: "var(--text-secondary)",
                                  }}
                                >
                                  {ev}
                                </span>
                              ))}
                            </div>
                          </div>

                          <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
                            <button
                              onClick={() => {
                                handleSelectWebhook(hook.id);
                                handleTestDispatch();
                              }}
                              disabled={dispatching}
                              className="btn btn-secondary"
                              style={{ display: "inline-flex", alignItems: "center", gap: "6px", fontSize: "0.8rem" }}
                            >
                              <Send size={14} color="var(--neon-cyan)" />
                              {dispatching && selectedWebhookId === hook.id ? "Signing & Dispatching..." : "Test Ping"}
                            </button>
                          </div>
                        </div>
                      );
                    })}
                  </div>
                )}
              </div>

              {/* Delivery History & Audit Log */}
              <div className="card" style={{ padding: "24px" }}>
                <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "16px" }}>
                  <h4 style={{ margin: 0, fontSize: "1rem", color: "#fff", display: "flex", alignItems: "center", gap: "8px" }}>
                    <Radio size={16} color="var(--neon-cyan)" /> Recent Deliveries & HMAC Verification
                  </h4>
                  <span style={{ fontSize: "0.75rem", color: "var(--text-muted)" }}>
                    Header: <code>X-Vyoma-Signature-256</code>
                  </span>
                </div>

                {deliveries.length === 0 ? (
                  <div style={{ padding: "24px", textAlign: "center", color: "var(--text-muted)", fontSize: "0.85rem" }}>
                    No delivery records found. Click <strong>Test Ping</strong> above to verify webhook HMAC signing.
                  </div>
                ) : (
                  <div style={{ display: "flex", flexDirection: "column", gap: "10px" }}>
                    {deliveries.map((del) => (
                      <div
                        key={del.id}
                        style={{
                          padding: "14px",
                          background: "rgba(0, 0, 0, 0.3)",
                          border: "1px solid var(--border-subtle)",
                          borderRadius: "8px",
                          fontFamily: "var(--font-mono)",
                          fontSize: "0.8rem",
                        }}
                      >
                        <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "8px" }}>
                          <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                            <span
                              style={{
                                color: del.status_code === 200 ? "#10b981" : "#ef4444",
                                fontWeight: 700,
                              }}
                            >
                              HTTP {del.status_code || 200} OK
                            </span>
                            <span style={{ color: "var(--text-muted)" }}>•</span>
                            <span style={{ color: "var(--neon-cyan)" }}>Event: {del.event_type}</span>
                          </div>
                          <span style={{ color: "var(--text-muted)", fontSize: "0.75rem" }}>
                            {new Date(del.delivered_at).toLocaleTimeString()}
                          </span>
                        </div>

                        <div style={{
                          background: "rgba(0, 0, 0, 0.4)",
                          padding: "8px 12px",
                          borderRadius: "6px",
                          color: "var(--text-secondary)",
                          overflowX: "auto",
                        }}>
                          {JSON.stringify(del.request_payload)}
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </div>
          )}

          {/* SUBTAB 3: TRUST & POLICIES */}
          {activeSubTab === "settings" && (
            <div className="card" style={{ padding: "24px", display: "flex", flexDirection: "column", gap: "20px" }}>
              <h3 style={{ margin: "0 0 8px 0", fontSize: "1.2rem", color: "#fff" }}>
                Enterprise Registry Trust Configuration
              </h3>

              <div style={{ display: "flex", flexDirection: "column", gap: "16px" }}>
                <div style={{
                  padding: "16px",
                  background: "rgba(255, 255, 255, 0.02)",
                  border: "1px solid var(--border-subtle)",
                  borderRadius: "10px",
                  display: "flex",
                  justifyContent: "space-between",
                  alignItems: "center",
                }}>
                  <div>
                    <div style={{ fontWeight: 600, color: "#fff", marginBottom: "4px" }}>
                      Strict Ed25519 Notary Enforcement
                    </div>
                    <div style={{ color: "var(--text-muted)", fontSize: "0.85rem" }}>
                      Reject appliance pulls if the manifest signature does not chain back to the Vyoma Root Authority.
                    </div>
                  </div>
                  <span className="badge badge-emerald">Enforced</span>
                </div>

                <div style={{
                  padding: "16px",
                  background: "rgba(255, 255, 255, 0.02)",
                  border: "1px solid var(--border-subtle)",
                  borderRadius: "10px",
                  display: "flex",
                  justifyContent: "space-between",
                  alignItems: "center",
                }}>
                  <div>
                    <div style={{ fontWeight: 600, color: "#fff", marginBottom: "4px" }}>
                      CVE Vulnerability Gatekeeper
                    </div>
                    <div style={{ color: "var(--text-muted)", fontSize: "0.85rem" }}>
                      Automatically block deployment of microVMs that contain unresolved Critical severity CVEs.
                    </div>
                  </div>
                  <span className="badge badge-emerald">Active (Scorecard Gate)</span>
                </div>

                <div style={{
                  padding: "16px",
                  background: "rgba(255, 255, 255, 0.02)",
                  border: "1px solid var(--border-subtle)",
                  borderRadius: "10px",
                  display: "flex",
                  justifyContent: "space-between",
                  alignItems: "center",
                }}>
                  <div>
                    <div style={{ fontWeight: 600, color: "#fff", marginBottom: "4px" }}>
                      Private Namespace Isolation
                    </div>
                    <div style={{ color: "var(--text-muted)", fontSize: "0.85rem" }}>
                      Allow anonymous pulls for public catalog entries while enforcing PAT authentication for private appliances.
                    </div>
                  </div>
                  <span className="badge badge-cyan">Enabled</span>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* MODAL: INVITE MEMBER */}
      {showInviteModal && (
        <div
          style={{
            position: "fixed",
            inset: 0,
            background: "rgba(0, 0, 0, 0.8)",
            backdropFilter: "blur(8px)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            zIndex: 1000,
            padding: "20px",
          }}
          onClick={() => setShowInviteModal(false)}
        >
          <div
            className="card"
            style={{
              maxWidth: "480px",
              width: "100%",
              padding: "28px",
              background: "#0c101b",
              border: "1px solid var(--neon-cyan)",
              boxShadow: "0 0 30px rgba(0, 242, 254, 0.2)",
            }}
            onClick={(e) => e.stopPropagation()}
          >
            <h3 style={{ margin: "0 0 8px 0", color: "#fff", fontSize: "1.3rem" }}>
              Invite Team Member to @{selectedOrg?.name}
            </h3>
            <p style={{ color: "var(--text-muted)", fontSize: "0.85rem", marginBottom: "20px" }}>
              Add a user and assign role-based permissions for this organization.
            </p>

            <form onSubmit={handleInviteSubmit}>
              <div style={{ marginBottom: "16px" }}>
                <label style={{ display: "block", color: "var(--text-secondary)", fontSize: "0.85rem", marginBottom: "6px" }}>
                  Username
                </label>
                <input
                  type="text"
                  required
                  placeholder="e.g. dev-lead or sec-auditor"
                  value={inviteUsername}
                  onChange={(e) => setInviteUsername(e.target.value)}
                  style={{
                    width: "100%",
                    padding: "10px 14px",
                    background: "rgba(0, 0, 0, 0.4)",
                    border: "1px solid var(--border-subtle)",
                    borderRadius: "8px",
                    color: "#fff",
                    outline: "none",
                  }}
                />
              </div>

              <div style={{ marginBottom: "24px" }}>
                <label style={{ display: "block", color: "var(--text-secondary)", fontSize: "0.85rem", marginBottom: "6px" }}>
                  Role & Access Level
                </label>
                <select
                  value={inviteRole}
                  onChange={(e) => setInviteRole(e.target.value)}
                  style={{
                    width: "100%",
                    padding: "10px 14px",
                    background: "rgba(0, 0, 0, 0.4)",
                    border: "1px solid var(--border-subtle)",
                    borderRadius: "8px",
                    color: "#fff",
                    outline: "none",
                  }}
                >
                  <option value="member">Member (Pull & Push Appliances)</option>
                  <option value="admin">Admin (Manage Webhooks & Members)</option>
                  <option value="owner">Owner (Full Governance & Signing)</option>
                </select>
              </div>

              <div style={{ display: "flex", justifyContent: "flex-end", gap: "10px" }}>
                <button
                  type="button"
                  onClick={() => setShowInviteModal(false)}
                  className="btn btn-secondary"
                >
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary">
                  Confirm Invitation
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* MODAL: ADD WEBHOOK */}
      {showWebhookModal && (
        <div
          style={{
            position: "fixed",
            inset: 0,
            background: "rgba(0, 0, 0, 0.8)",
            backdropFilter: "blur(8px)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            zIndex: 1000,
            padding: "20px",
          }}
          onClick={() => setShowWebhookModal(false)}
        >
          <div
            className="card"
            style={{
              maxWidth: "520px",
              width: "100%",
              padding: "28px",
              background: "#0c101b",
              border: "1px solid var(--neon-cyan)",
              boxShadow: "0 0 30px rgba(0, 242, 254, 0.2)",
            }}
            onClick={(e) => e.stopPropagation()}
          >
            <h3 style={{ margin: "0 0 8px 0", color: "#fff", fontSize: "1.3rem" }}>
              Configure Signed Webhook
            </h3>
            <p style={{ color: "var(--text-muted)", fontSize: "0.85rem", marginBottom: "20px" }}>
              Paylods are signed using HMAC-SHA256 with header <code>X-Vyoma-Signature-256</code>.
            </p>

            <form onSubmit={handleWebhookSubmit}>
              <div style={{ marginBottom: "16px" }}>
                <label style={{ display: "block", color: "var(--text-secondary)", fontSize: "0.85rem", marginBottom: "6px" }}>
                  Endpoint URL
                </label>
                <input
                  type="url"
                  required
                  placeholder="https://ci.example.com/webhooks/vyoma"
                  value={webhookUrl}
                  onChange={(e) => setWebhookUrl(e.target.value)}
                  style={{
                    width: "100%",
                    padding: "10px 14px",
                    background: "rgba(0, 0, 0, 0.4)",
                    border: "1px solid var(--border-subtle)",
                    borderRadius: "8px",
                    color: "#fff",
                    outline: "none",
                  }}
                />
              </div>

              <div style={{ marginBottom: "16px" }}>
                <label style={{ display: "block", color: "var(--text-secondary)", fontSize: "0.85rem", marginBottom: "6px" }}>
                  HMAC Shared Secret (Optional - auto-generated if blank)
                </label>
                <input
                  type="text"
                  placeholder="whsec_xxxxxxxxxxxxxxxxxxxxxx"
                  value={webhookSecret}
                  onChange={(e) => setWebhookSecret(e.target.value)}
                  style={{
                    width: "100%",
                    padding: "10px 14px",
                    background: "rgba(0, 0, 0, 0.4)",
                    border: "1px solid var(--border-subtle)",
                    borderRadius: "8px",
                    color: "#fff",
                    fontFamily: "var(--font-mono)",
                    fontSize: "0.85rem",
                    outline: "none",
                  }}
                />
              </div>

              <div style={{ marginBottom: "24px" }}>
                <label style={{ display: "block", color: "var(--text-secondary)", fontSize: "0.85rem", marginBottom: "8px" }}>
                  Trigger Events
                </label>
                <div style={{ display: "flex", gap: "12px", flexWrap: "wrap" }}>
                  {["push", "release", "scan"].map((ev) => (
                    <label
                      key={ev}
                      style={{
                        display: "flex",
                        alignItems: "center",
                        gap: "6px",
                        fontSize: "0.85rem",
                        color: "#fff",
                        cursor: "pointer",
                      }}
                    >
                      <input
                        type="checkbox"
                        checked={webhookEvents.includes(ev)}
                        onChange={(e) => {
                          if (e.target.checked) {
                            setWebhookEvents((prev) => [...prev, ev]);
                          } else {
                            setWebhookEvents((prev) => prev.filter((item) => item !== ev));
                          }
                        }}
                      />
                      <code>{ev}</code>
                    </label>
                  ))}
                </div>
              </div>

              <div style={{ display: "flex", justifyContent: "flex-end", gap: "10px" }}>
                <button
                  type="button"
                  onClick={() => setShowWebhookModal(false)}
                  className="btn btn-secondary"
                >
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary">
                  Create Webhook
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};
