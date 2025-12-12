"""
Bevy Material UI - Documentation Screenshot Capture
====================================================

Captures clean screenshots of each component section for documentation.
Uses the existing telemetry infrastructure for navigation.

Usage:
    python capture_docs.py              # Capture all sections
    python capture_docs.py --section buttons  # Capture specific section
    python capture_docs.py --list       # List available sections

Output:
    docs/components/screenshots/<section>.png
"""

import subprocess
import time
import sys
import argparse
from pathlib import Path
from datetime import datetime

try:
    import pyautogui
    from PIL import ImageGrab, Image
except ImportError:
    subprocess.run([sys.executable, "-m", "pip", "install", "pyautogui", "pillow"], check=True)
    import pyautogui
    from PIL import ImageGrab, Image

# Import from quick_test for shared functionality
from quick_test import (
    read_telemetry, 
    click_element, 
    find_bevy_window, 
    set_window_bounds,
    get_element_bounds,
    WORKSPACE_DIR,
)

pyautogui.FAILSAFE = False
pyautogui.PAUSE = 0.1

# Output directory for documentation screenshots
DOCS_SCREENSHOTS_DIR = WORKSPACE_DIR / "docs" / "components" / "screenshots"

# Section navigation mapping: nav element test_id -> (section_name, display_name)
SECTIONS = {
    "nav_buttons": ("button", "Buttons"),
    "nav_checkboxes": ("checkbox", "Checkboxes"),
    "nav_switches": ("switch", "Switches"),
    "nav_radio": ("radio", "Radio Buttons"),
    "nav_chips": ("chip", "Chips"),
    "nav_fabs": ("fab", "FABs"),
    "nav_icon_buttons": ("icon_button", "Icon Buttons"),
    "nav_sliders": ("slider", "Sliders"),
    "nav_text_fields": ("text_field", "Text Fields"),
    "nav_dialogs": ("dialog", "Dialogs"),
    "nav_menus": ("menu", "Menus"),
    "nav_lists": ("list", "Lists"),
    "nav_cards": ("card", "Cards"),
    "nav_tooltips": ("tooltip", "Tooltips"),
    "nav_snackbars": ("snackbar", "Snackbars"),
    "nav_tabs": ("tabs", "Tabs"),
    "nav_progress": ("progress", "Progress"),
    "nav_badges": ("badge", "Badges"),
    "nav_dividers": ("divider", "Dividers"),
}


def ensure_output_dir():
    """Create the output directory if it doesn't exist"""
    DOCS_SCREENSHOTS_DIR.mkdir(parents=True, exist_ok=True)
    print(f"📁 Output directory: {DOCS_SCREENSHOTS_DIR}")


def start_showcase():
    """Start the showcase application with telemetry enabled"""
    print("\n🚀 Starting showcase application...")
    
    # Start the showcase with telemetry
    process = subprocess.Popen(
        ["cargo", "run", "--example", "showcase", "--", "--telemetry"],
        cwd=WORKSPACE_DIR,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    
    # Wait for app to start
    print("   Waiting for application to initialize...")
    time.sleep(5)
    
    return process


def wait_for_window(timeout: float = 30.0) -> tuple:
    """Wait for the Bevy window to appear and return its bounds"""
    print("   Looking for Bevy window...")
    start = time.time()
    
    while time.time() - start < timeout:
        client_origin, window_rect = find_bevy_window(maximize=True)
        if window_rect:
            print(f"   ✓ Window found at {window_rect}")
            return client_origin, window_rect
        time.sleep(1)
    
    print("   ✗ Window not found within timeout")
    return None, None


def navigate_to_section(nav_id: str, section_name: str) -> bool:
    """Navigate to a section by clicking its nav element"""
    print(f"   Navigating to {section_name}...")
    
    # Try to click the nav element
    if click_element(nav_id):
        time.sleep(0.5)  # Wait for UI to update
        return True
    
    print(f"   ✗ Could not find nav element: {nav_id}")
    return False


def capture_section_screenshot(section_id: str, window_rect: tuple) -> Path:
    """Capture a screenshot of the current section"""
    filepath = DOCS_SCREENSHOTS_DIR / f"{section_id}.png"
    
    # Capture the window area
    if window_rect:
        img = ImageGrab.grab(bbox=window_rect)
    else:
        img = ImageGrab.grab()
    
    # Save the screenshot
    img.save(filepath)
    print(f"   📸 Saved: {filepath.name}")
    
    return filepath


def capture_cropped_section(section_id: str, window_rect: tuple, 
                           crop_sidebar: bool = True) -> Path:
    """Capture a cropped screenshot focusing on the main content area"""
    filepath = DOCS_SCREENSHOTS_DIR / f"{section_id}.png"
    
    # Capture the window area
    if window_rect:
        img = ImageGrab.grab(bbox=window_rect)
    else:
        img = ImageGrab.grab()
    
    # Crop to remove sidebar (approximately left 280px is sidebar)
    if crop_sidebar and img.width > 400:
        # Crop: remove sidebar on left, keep main content
        sidebar_width = 280
        top_bar_height = 60  # Title bar area
        img = img.crop((sidebar_width, top_bar_height, img.width, img.height))
    
    # Save the screenshot
    img.save(filepath)
    print(f"   📸 Saved: {filepath.name} ({img.width}x{img.height})")
    
    return filepath


def capture_all_sections(window_rect: tuple, crop: bool = True):
    """Capture screenshots of all component sections"""
    print("\n📷 Capturing all sections...")
    
    captured = []
    failed = []
    
    for nav_id, (section_id, display_name) in SECTIONS.items():
        print(f"\n[{section_id}] {display_name}")
        
        # Navigate to section
        if not navigate_to_section(nav_id, display_name):
            failed.append(section_id)
            continue
        
        # Wait for content to render
        time.sleep(0.3)
        
        # Capture screenshot
        if crop:
            filepath = capture_cropped_section(section_id, window_rect)
        else:
            filepath = capture_section_screenshot(section_id, window_rect)
        
        captured.append(section_id)
    
    return captured, failed


def capture_single_section(section_name: str, window_rect: tuple, crop: bool = True):
    """Capture a single section by name"""
    # Find the section
    for nav_id, (section_id, display_name) in SECTIONS.items():
        if section_id == section_name or display_name.lower() == section_name.lower():
            print(f"\n📷 Capturing {display_name}...")
            
            if navigate_to_section(nav_id, display_name):
                time.sleep(0.3)
                if crop:
                    capture_cropped_section(section_id, window_rect)
                else:
                    capture_section_screenshot(section_id, window_rect)
                return True
            return False
    
    print(f"Section not found: {section_name}")
    print("Available sections:", ", ".join(s[0] for s in SECTIONS.values()))
    return False


def list_sections():
    """Print available sections"""
    print("\nAvailable sections:")
    print("-" * 40)
    for nav_id, (section_id, display_name) in SECTIONS.items():
        print(f"  {section_id:15} - {display_name}")


def main():
    parser = argparse.ArgumentParser(
        description="Capture documentation screenshots for Bevy Material UI components"
    )
    parser.add_argument(
        "--section", "-s",
        help="Capture a specific section (e.g., 'button', 'checkbox')"
    )
    parser.add_argument(
        "--list", "-l",
        action="store_true",
        help="List available sections"
    )
    parser.add_argument(
        "--no-crop",
        action="store_true",
        help="Don't crop the sidebar from screenshots"
    )
    parser.add_argument(
        "--no-start",
        action="store_true",
        help="Don't start the showcase app (assume it's already running)"
    )
    
    args = parser.parse_args()
    
    if args.list:
        list_sections()
        return
    
    print("=" * 60)
    print("Bevy Material UI - Documentation Screenshot Capture")
    print("=" * 60)
    
    ensure_output_dir()
    
    process = None
    try:
        # Start showcase if needed
        if not args.no_start:
            process = start_showcase()
        
        # Find the window
        client_origin, window_rect = wait_for_window()
        if not window_rect:
            print("ERROR: Could not find the showcase window")
            if process:
                process.terminate()
            return 1
        
        # Wait a bit more for initial render
        time.sleep(1)
        
        crop = not args.no_crop
        
        if args.section:
            # Capture single section
            capture_single_section(args.section, window_rect, crop)
        else:
            # Capture all sections
            captured, failed = capture_all_sections(window_rect, crop)
            
            print("\n" + "=" * 60)
            print("SUMMARY")
            print("=" * 60)
            print(f"✓ Captured: {len(captured)} sections")
            if failed:
                print(f"✗ Failed: {len(failed)} sections - {', '.join(failed)}")
            print(f"\nScreenshots saved to: {DOCS_SCREENSHOTS_DIR}")
    
    finally:
        # Clean up
        if process:
            print("\n🛑 Stopping showcase application...")
            process.terminate()
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                process.kill()


if __name__ == "__main__":
    sys.exit(main() or 0)
