"use client";

import React, { useState, useEffect } from "react";
import { Navbar, HubTab } from "@/components/Navbar";
import { ApplianceCard } from "@/components/ApplianceCard";
import { ApplianceModal } from "@/components/ApplianceModal";
import { KernelExplorer } from "@/components/KernelExplorer";
import { SecurityNotaryTab } from "@/components/SecurityNotaryTab";
import { TokensManager } from "@/components/TokensManager";
import { OrganizationsTab } from "@/components/OrganizationsTab";
import { CatalogItem, KernelProfile } from "@/types/hub";
import {
  getCatalog,
  getKernels,
  checkBackendHealth,
  FALLBACK_APPLIANCES,
  FALLBACK_KERNELS,
} from "@/lib/api";
import {
  Layers,
  Cpu,
  ShieldCheck,
  Zap,
  Activity,
  Server,
} from "lucide-react";

export default function Home() {
  const [activeTab, setActiveTab] = useState<HubTab>("store");
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedCategory, setSelectedCategory] = useState("all");
  const [selectedAppliance, setSelectedAppliance] = useState<CatalogItem | null>(null);
  const [selectedKernelName, setSelectedKernelName] = useState<string | null>(null);

  const [appliances, setAppliances] = useState<CatalogItem[]>(FALLBACK_APPLIANCES);
  const [kernels, setKernels] = useState<KernelProfile[]>(FALLBACK_KERNELS);
  const [isBackendConnected, setIsBackendConnected] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  // Load data on mount & poll health
  useEffect(() => {
    async function loadData() {
      setIsLoading(true);
      const isHealthy = await checkBackendHealth();
      setIsBackendConnected(isHealthy);

      const [catResult, kernResult] = await Promise.all([
        getCatalog(searchQuery, selectedCategory),
        getKernels(),
      ]);

      setAppliances(catResult.items);
      setKernels(kernResult.kernels);
      setIsLoading(false);
    }

    loadData();
  }, [searchQuery, selectedCategory]);

  const handleSelectKernelFromAppliance = (kernelName: string) => {
    setSelectedKernelName(kernelName);
    setActiveTab("kernels");
  };

  const categories = [
    { id: "all", label: "All Workloads" },
    { id: "database", label: "Databases & Storage" },
    { id: "network", label: "Web & Edge Proxies" },
    { id: "ai", label: "AI & ML Inference" },
    { id: "runtime", label: "Application Runtimes" },
  ];

  return (
    <div style={{ minHeight: "100vh", display: "flex", flexDirection: "column" }}>
      {/* Sticky Global Navigation */}
      <Navbar
        activeTab={activeTab}
        onSelectTab={setActiveTab}
        searchQuery={searchQuery}
        onSearchChange={setSearchQuery}
        isBackendConnected={isBackendConnected}
      />

      {/* Main Content Area */}
      <main style={{ flex: 1, padding: "40px 24px", maxWidth: "1400px", width: "100%", margin: "0 auto" }}>
        {/* TAB 1: APPLIANCE STORE */}
        {activeTab === "store" && (
          <div>
            {/* Hero Section */}
            <div style={{ marginBottom: "48px", textAlign: "left" }}>
              <div style={{ display: "inline-flex", alignItems: "center", gap: "8px", marginBottom: "14px" }}>
                <span className="badge badge-cyan">
                  <Layers size={13} /> Official MicroVM Registry
                </span>
                <span className="badge badge-emerald">
                  <ShieldCheck size={13} /> Ed25519 Notary Verified
                </span>
              </div>

              <h1 style={{
                fontSize: "clamp(2rem, 4vw, 3rem)",
                fontWeight: 800,
                color: "#ffffff",
                letterSpacing: "-0.03em",
                lineHeight: "1.15",
                marginBottom: "16px",
                maxWidth: "900px",
              }}>
                Hardware-Isolated Micro-OS Appliances. Booting in{" "}
                <span style={{
                  background: "linear-gradient(135deg, #00f2fe 0%, #4facfe 100%)",
                  WebkitBackgroundClip: "text",
                  WebkitTextFillColor: "transparent",
                }}>
                  &lt; 250ms
                </span>{" "}
                with Workload-Tuned Linux Kernels.
              </h1>

              <p style={{
                fontSize: "1.1rem",
                color: "var(--text-secondary)",
                maxWidth: "760px",
                lineHeight: "1.6",
                marginBottom: "28px",
              }}>
                Browse production-ready appliances pre-configured with Cloud Hypervisor microVMs. Each appliance runs inside its own dedicated guest kernel with hardware KVM isolation and zero container escapes.
              </p>

              {/* Stat Highlights Pill */}
              <div style={{
                display: "inline-flex",
                flexWrap: "wrap",
                gap: "20px",
                background: "rgba(19, 24, 38, 0.6)",
                border: "1px solid var(--border-subtle)",
                borderRadius: "var(--radius-lg)",
                padding: "14px 22px",
              }}>
                <div style={{ display: "flex", alignItems: "center", gap: "8px", fontSize: "0.85rem" }}>
                  <Zap size={16} color="var(--accent-gold)" />
                  <span style={{ color: "#ffffff", fontWeight: 700 }}>&lt; 220ms</span>
                  <span style={{ color: "var(--text-muted)" }}>Cold Boot</span>
                </div>
                <div style={{ width: "1px", height: "18px", background: "var(--border-subtle)" }} />
                <div style={{ display: "flex", alignItems: "center", gap: "8px", fontSize: "0.85rem" }}>
                  <Activity size={16} color="var(--accent-cyan)" />
                  <span style={{ color: "#ffffff", fontWeight: 700 }}>~26 MB</span>
                  <span style={{ color: "var(--text-muted)" }}>Host Memory Footprint</span>
                </div>
                <div style={{ width: "1px", height: "18px", background: "var(--border-subtle)" }} />
                <div style={{ display: "flex", alignItems: "center", gap: "8px", fontSize: "0.85rem" }}>
                  <Cpu size={16} color="var(--accent-emerald)" />
                  <span style={{ color: "#ffffff", fontWeight: 700 }}>5 Workload</span>
                  <span style={{ color: "var(--text-muted)" }}>Certified Kernels</span>
                </div>
              </div>
            </div>

            {/* Filter Bar */}
            <div style={{
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              flexWrap: "wrap",
              gap: "14px",
              marginBottom: "32px",
              paddingBottom: "20px",
              borderBottom: "1px solid var(--border-subtle)",
            }}>
              <div style={{ display: "flex", alignItems: "center", gap: "8px", flexWrap: "wrap" }}>
                {categories.map((cat) => (
                  <button
                    key={cat.id}
                    onClick={() => setSelectedCategory(cat.id)}
                    style={{
                      padding: "8px 16px",
                      borderRadius: "var(--radius-md)",
                      fontSize: "0.85rem",
                      fontWeight: 600,
                      cursor: "pointer",
                      transition: "all 0.2s ease",
                      background: selectedCategory === cat.id ? "var(--accent-cyan)" : "rgba(255, 255, 255, 0.03)",
                      color: selectedCategory === cat.id ? "#040914" : "var(--text-secondary)",
                      border: `1px solid ${selectedCategory === cat.id ? "var(--accent-cyan)" : "var(--border-subtle)"}`,
                    }}
                  >
                    {cat.label}
                  </button>
                ))}
              </div>

              <div style={{ fontSize: "0.85rem", color: "var(--text-muted)" }}>
                Showing <strong style={{ color: "#ffffff" }}>{isLoading ? "loading..." : appliances.length}</strong> appliances
              </div>
            </div>

            {/* Appliances Grid */}
            <div style={{
              display: "grid",
              gridTemplateColumns: "repeat(auto-fill, minmax(360px, 1fr))",
              gap: "24px",
            }}>
              {appliances.map((item) => (
                <ApplianceCard
                  key={item.id}
                  item={item}
                  onOpenDetails={setSelectedAppliance}
                  onSelectKernel={handleSelectKernelFromAppliance}
                />
              ))}
            </div>
          </div>
        )}

        {/* TAB 2: CERTIFIED KERNELS */}
        {activeTab === "kernels" && (
          <KernelExplorer
            kernels={kernels}
            selectedKernelName={selectedKernelName}
          />
        )}

        {/* TAB 3: SECURITY & NOTARY */}
        {activeTab === "notary" && <SecurityNotaryTab />}

        {/* TAB 4: ENTERPRISE ORGANIZATIONS & RBAC */}
        {activeTab === "orgs" && <OrganizationsTab />}

        {/* TAB 5: TOKENS & CLI ACCESS */}
        {activeTab === "tokens" && <TokensManager />}
      </main>

      {/* Modal for detailed appliance view */}
      <ApplianceModal
        item={selectedAppliance}
        onClose={() => setSelectedAppliance(null)}
        onSelectKernel={handleSelectKernelFromAppliance}
      />

      {/* Global Footer */}
      <footer style={{
        marginTop: "auto",
        borderTop: "1px solid var(--border-subtle)",
        background: "#05070c",
        padding: "40px 24px",
      }}>
        <div style={{
          maxWidth: "1400px",
          margin: "0 auto",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          flexWrap: "wrap",
          gap: "20px",
        }}>
          <div>
            <div style={{ display: "flex", alignItems: "center", gap: "8px", marginBottom: "6px" }}>
              <span style={{ fontWeight: 800, fontSize: "1.05rem", color: "#ffffff" }}>
                Vyoma<span style={{ color: "var(--accent-cyan)" }}>Hub</span>
              </span>
              <span className="badge badge-cyan" style={{ fontSize: "0.68rem" }}>
                Standalone SaaS Registry
              </span>
            </div>
            <p style={{ fontSize: "0.8rem", color: "var(--text-muted)" }}>
              The Official App Store for Hardware-Isolated Micro-OS Appliances & Certified Linux Kernels.
            </p>
          </div>

          <div style={{ display: "flex", alignItems: "center", gap: "24px", fontSize: "0.85rem", color: "var(--text-secondary)" }}>
            <span style={{ display: "flex", alignItems: "center", gap: "6px" }}>
              <Server size={14} color="var(--accent-emerald)" /> Cloud Hypervisor v54.0
            </span>
            <span style={{ display: "flex", alignItems: "center", gap: "6px" }}>
              <ShieldCheck size={14} color="var(--accent-gold)" /> Ed25519 Notary Verified
            </span>
            <span style={{ color: "var(--text-muted)" }}>
              Apache 2.0 Open Source
            </span>
          </div>
        </div>
      </footer>
    </div>
  );
}
