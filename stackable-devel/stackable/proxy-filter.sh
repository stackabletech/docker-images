#!/bin/bash

# HTTP Proxy filter script for Stackable builds
# Starts squid proxy filtering and provides logging utilities

set -euo pipefail

PROXY_PORT=3128
PROXY_PID_FILE="/var/run/squid.pid"
PROXY_LOG_FILE="/var/log/squid/access.log"

# Function to start proxy filtering
start_proxy_filter() {
    echo "🔒 Starting HTTP proxy filtering for Stackable builds..."
    
    # Check if already running
    if [ -f "$PROXY_PID_FILE" ] && kill -0 "$(cat "$PROXY_PID_FILE")" 2>/dev/null; then
        echo "⚠️  Proxy already running (PID: $(cat "$PROXY_PID_FILE"))"
        return 0
    fi
    
    # Start squid (foreground mode)
    squid -N &
    
    # Wait for startup
    sleep 2
    
    # Verify it's running
    if [ -f "$PROXY_PID_FILE" ] && kill -0 "$(cat "$PROXY_PID_FILE")" 2>/dev/null; then
        PROXY_PID=$(cat "$PROXY_PID_FILE")
        echo "✅ HTTP proxy filtering active (PID: $PROXY_PID, Port: $PROXY_PORT)"
        echo "🔍 HTTP requests will be logged to $PROXY_LOG_FILE"
        
        echo "📡 Make sure proxy environment variables are set:"
        echo "   HTTP_PROXY=$HTTP_PROXY"
        echo "   HTTPS_PROXY=$HTTPS_PROXY"
    else
        echo "❌ Failed to start proxy"
        exit 1
    fi
}

# Function to stop proxy filtering
stop_proxy_filter() {
    if [ -f "$PROXY_PID_FILE" ]; then
        PID=$(cat "$PROXY_PID_FILE")
        if kill -0 "$PID" 2>/dev/null; then
            kill "$PID"
            rm -f "$PROXY_PID_FILE"
            echo "🔓 HTTP proxy filtering stopped"
        else
            echo "ℹ️  Proxy was not running"
            rm -f "$PROXY_PID_FILE"
        fi
    else
        echo "ℹ️  Proxy was not running"
    fi
    
    # Unset proxy environment variables
    unset HTTP_PROXY HTTPS_PROXY http_proxy https_proxy || true
    echo "📡 Proxy environment variables cleared"
}

# Function to show proxy access log
show_log() {
    if [ -f "$PROXY_LOG_FILE" ]; then
        echo "📋 Recent HTTP proxy requests:"
        tail -20 "$PROXY_LOG_FILE"
    else
        echo "ℹ️  No proxy log found"
    fi
}

# Function to show blocked requests
show_blocked() {
    local DENIED_LOG_FILE="/var/log/squid/denied.log"
    
    if [ -f "$DENIED_LOG_FILE" ]; then
        echo "🚫 Blocked HTTP requests (from denied.log):"
        tail -20 "$DENIED_LOG_FILE"
    elif [ -f "$PROXY_LOG_FILE" ]; then
        echo "🚫 Blocked HTTP requests (from access.log):"
        grep -E "(TCP_DENIED|DENIED)" "$PROXY_LOG_FILE" | tail -10 || echo "No blocked requests found in access log"
    else
        echo "ℹ️  No proxy logs found"
    fi
}

# Function to show allowed requests
show_allowed() {
    if [ -f "$PROXY_LOG_FILE" ]; then
        echo "✅ Allowed HTTP requests:"
        grep -E "(CONNECT|GET|POST)" "$PROXY_LOG_FILE" | grep -v -E "(DENIED|ERROR|refused)" | tail -10 || echo "No allowed requests found"
    else
        echo "ℹ️  No proxy log found"
    fi
}

# Function to test proxy filtering
test_proxy() {
    echo "🧪 Testing HTTP proxy filtering..."
    
    if ! [ -f "$PROXY_PID_FILE" ] || ! kill -0 "$(cat "$PROXY_PID_FILE")" 2>/dev/null; then
        echo "❌ Proxy not running - start it first with: proxy-filter.sh start"
        return 1
    fi
    
    export HTTP_PROXY="http://127.0.0.1:$PROXY_PORT"
    export HTTPS_PROXY="http://127.0.0.1:$PROXY_PORT"
    
    echo "✅ Testing allowed domain: stackable.tech"
    if timeout 10 curl -s -o /dev/null -w "%{http_code}" --proxy "$HTTP_PROXY" https://stackable.tech >/dev/null 2>&1; then
        echo "   ✅ stackable.tech - ALLOWED"
    else
        echo "   ⚠️  stackable.tech - Failed (may be normal)"
    fi
    
    echo "❌ Testing blocked domain: registry.npmjs.org"
    if timeout 5 curl -s -o /dev/null --proxy "$HTTP_PROXY" https://registry.npmjs.org >/dev/null 2>&1; then
        echo "   ❌ registry.npmjs.org - INCORRECTLY ALLOWED"
    else
        echo "   ✅ registry.npmjs.org - CORRECTLY BLOCKED"
    fi
}

# Function to show status
show_status() {
    if [ -f "$PROXY_PID_FILE" ] && kill -0 "$(cat "$PROXY_PID_FILE")" 2>/dev/null; then
        PID=$(cat "$PROXY_PID_FILE")
        echo "✅ HTTP proxy filtering is ACTIVE"
        echo "   PID: $PID"
        echo "   Port: $PROXY_PORT"
        echo "   Log: $PROXY_LOG_FILE"
        echo "   Config: /etc/squid/squid.conf"
    else
        echo "❌ HTTP proxy filtering is INACTIVE"
    fi
}

# Main script logic
case "${1:-start}" in
    start)
        start_proxy_filter
        ;;
    stop)
        stop_proxy_filter
        ;;
    restart)
        stop_proxy_filter
        sleep 1
        start_proxy_filter
        ;;
    status)
        show_status
        ;;
    log)
        show_log
        ;;
    blocked)
        show_blocked
        ;;
    allowed)
        show_allowed
        ;;
    test)
        test_proxy
        ;;
    help|--help|-h)
        echo "HTTP Proxy Filter for Stackable Builds"
        echo "Usage: $0 [start|stop|restart|status|log|blocked|allowed|test]"
        echo ""
        echo "Commands:"
        echo "  start   - Start proxy filtering (default)"
        echo "  stop    - Stop proxy filtering"
        echo "  restart - Restart proxy filtering"
        echo "  status  - Show proxy status"
        echo "  log     - Show recent HTTP requests"
        echo "  blocked - Show blocked HTTP requests"
        echo "  allowed - Show allowed HTTP requests"
        echo "  test    - Test proxy filtering is working"
        echo ""
        echo "Usage in Dockerfiles:"
        echo "  RUN proxy-filter.sh start && \\"
        echo "      HTTP_PROXY=http://127.0.0.1:$PROXY_PORT \\"
        echo "      HTTPS_PROXY=http://127.0.0.1:$PROXY_PORT \\"
        echo "      your-build-command && \\"
        echo "      proxy-filter.sh log"
        ;;
    *)
        echo "Unknown command: $1"
        echo "Use '$0 help' for usage information"
        exit 1
        ;;
esac
