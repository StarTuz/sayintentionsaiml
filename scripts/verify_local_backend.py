#!/usr/bin/env python3
import sys
import os
import time

# Add client source to path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "../client/src")))

from core.providers.factory import get_provider
from core.providers.local import LocalSpeechProvider

def main():
    print("=== Stratus Local Backend Verification ===")
    
    # 1. Test Factory
    print("\n[1] Testing Factory (expecting LocalSpeechProvider)...")
    provider = get_provider()
    print(f"    Got provider: {type(provider).__name__}")
    
    if not isinstance(provider, LocalSpeechProvider):
        print("FAIL: Factory did not return LocalSpeechProvider (check config.ini)")
        sys.exit(1)
        
    # 2. Test Connection
    print("\n[2] Testing D-Bus Connection...")
    if provider.connect():
        print("    SUCCESS: Connected to SpeechD-NG")
    else:
        print("    FAIL: Could not connect. Is speechserverdaemon running?")
        print("    (Expected if daemon is not active)")
        # We continue to show what would happen, but marks as fail
        
    # 3. Test Methods (Mock if needed, or real)
    if provider.connected:
        print("\n[3] Testing 'Think' (AI)...")
        resp = provider.think("System check.")
        print(f"    Response: {resp}")
        
        print("\n[4] Testing 'Say' (TTS)...")
        resp = provider.say("Verification complete.")
        print(f"    Response: {resp}")
    else:
        print("\n[Skipping method tests due to no connection]")

if __name__ == "__main__":
    main()
