#!/usr/bin/env bash
# Zed IDE Maximum Physical Optimization Setup Script for Arch Linux
# Target: Intel Core i5-13500H (Raptor Lake), Mesa anv Vulkan, dwl Wayland Compositor

set -euo pipefail

if [[ $EUID -ne 0 ]]; then
    echo "This script must be run with root privileges (sudo)." >&2
    exit 1
fi

echo "================================================================="
echo " Configuring Maximum Physical Hardware Optimizations for Zed IDE "
echo "================================================================="

# 1. Scheduler Preemption: PREEMPT_FULL for sub-millisecond input responsiveness
if [[ -f /sys/kernel/debug/sched/preempt ]]; then
    echo "Configuring kernel preempt scheduler..."
    echo full > /sys/kernel/debug/sched/preempt || true
    echo "Current preemption state: $(cat /sys/kernel/debug/sched/preempt)"

    # Persist via systemd one-shot service across reboots
    cat << 'EOF' > /etc/systemd/system/zed-sched-preempt.service
[Unit]
Description=Ensure full kernel preemption for low-latency desktop
After=local-fs.target

[Service]
Type=oneshot
ExecStart=/bin/sh -c 'if [ -f /sys/kernel/debug/sched/preempt ]; then echo full > /sys/kernel/debug/sched/preempt; fi'
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target
EOF
    systemctl daemon-reload
    systemctl enable --now zed-sched-preempt.service 2>/dev/null || true
fi

# 2. Memory Subsystem & Low-Latency Sysctl
echo "Applying low-latency virtual memory and inotify limits..."
cat << 'EOF' > /etc/sysctl.d/99-zed-physical-optimization.conf
# Maximum memory map count for mimalloc thread caches and rustc/clangd LSPs
vm.max_map_count = 2147483642

# High capacity inotify limits for large C++ and workspace file watching
fs.inotify.max_user_watches = 524288
fs.inotify.max_user_instances = 1024

# Aggressive dirty page flushing to avoid I/O stalls during large file saves
vm.dirty_background_ratio = 5
vm.dirty_ratio = 10

# Low-latency CFS bandwidth scheduler slice
kernel.sched_cfs_bandwidth_slice_us = 3000
EOF

sysctl -p /etc/sysctl.d/99-zed-physical-optimization.conf >/dev/null 2>&1 || sysctl --system >/dev/null 2>&1

# 3. Transparent Hugepages (THP) for Low Allocator TLB Overhead
if [[ -f /sys/kernel/mm/transparent_hugepage/enabled ]]; then
    echo "Configuring Transparent Hugepages..."
    echo always > /sys/kernel/mm/transparent_hugepage/enabled 2>/dev/null || true
fi
if [[ -f /sys/kernel/mm/transparent_hugepage/defrag ]]; then
    echo defer+madvise > /sys/kernel/mm/transparent_hugepage/defrag 2>/dev/null || true
fi

# 4. Intel Iris Xe Graphics (Mesa anv Vulkan) Permissions & DMA-BUF
echo "Checking Intel GPU render node permissions..."
chmod 666 /dev/dri/renderD* 2>/dev/null || true

echo "================================================================="
echo " System-level physical optimization setup complete!              "
echo "================================================================="
