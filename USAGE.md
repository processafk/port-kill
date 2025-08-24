# Port Kill - Usage Guide

## Quick Start

1. **Build the application:**
   ```bash
   ./install.sh
   ```

2. **Run the application (easy way):**
   ```bash
   ./run.sh
   ```

3. **Run manually (alternative):**
   ```bash
   ./target/release/port-kill
   ```

4. **Run with logging (for debugging):**
   ```bash
   RUST_LOG=info ./target/release/port-kill
   ```

5. **Look for the status bar icon**: A white square with colored center in your status bar

## Testing the Application

1. **Start test servers:**
   ```bash
   ./test_ports.sh
   ```

2. **Run port-kill in another terminal:**
   ```bash
   RUST_LOG=info ./target/release/port-kill
   ```

3. **Check the status bar icon** - you should see a white square with a red or orange center indicating processes are detected.

4. **Click the status bar icon** to see the context menu with:
   - Kill All Processes
   - Individual process entries (e.g., "Kill: Port 3000: python3 (PID 1234)")
   - Quit

## Features Demonstrated

### Real-time Process Detection
- Monitors ports 2000-6000 every 5 seconds
- Uses `lsof -i :PORT -sTCP:LISTEN` for accurate detection
- Updates status bar immediately when processes start/stop

### Status Bar Icon
- Shows white background with green center when no processes are running
- Shows white background with red center when 1-9 processes are detected
- Shows white background with orange center when 10+ processes are detected
- Tooltip shows exact process count and details

### Process Management
- **Kill All Processes**: Terminates all detected development processes
- **One-Click Killing**: Click any menu item to kill all processes (current implementation)
- **Safe Termination**: Uses SIGTERM first, then SIGKILL if needed
- **Background Processing**: Process killing runs in background threads to maintain UI responsiveness

### Dynamic Menu
- Menu updates every 3 seconds when processes change
- Each process entry shows port, process name, and PID
- Menu updates are throttled to prevent crashes
- Currently shows all processes but kills all when any item is clicked

## Troubleshooting

### Application Won't Start
- Check if another instance is running: `ps aux | grep port-kill`
- Kill existing instances: `pkill -f port-kill`
- Check permissions and dependencies

### No Processes Detected
- Verify processes are listening on TCP ports 2000-6000
- Check if processes are in LISTEN state
- Use `lsof -i :2000-6000` to manually verify

### Permission Errors
- Some system processes may be protected
- Check process ownership
- Ensure the application has necessary permissions

### Menu Not Updating
- Check if the status bar icon is visible
- Verify the application is running: `ps aux | grep port-kill`
- Restart the application if needed

## Logging Levels

- `RUST_LOG=error`: Only error messages
- `RUST_LOG=warn`: Warnings and errors
- `RUST_LOG=info`: Information, warnings, and errors (recommended)
- `RUST_LOG=debug`: All messages including debug information

## Architecture

The application uses a stable event-driven architecture:

1. **Main Thread**: Handles UI events and menu interactions with winit event loop
2. **Process Monitor**: Scans for processes every 5 seconds using `lsof`
3. **Menu Updates**: Updates context menu every 3 seconds when processes change
4. **Background Processing**: Process killing runs in separate threads to maintain UI responsiveness

## Port Range

The application monitors ports 2000-6000, which covers a broad range of development ports including:
- HTTP development servers (2000-2999)
- React development servers (3000)
- Node.js applications (3001, 3002, etc.)
- Python development servers (4000-4999, 8000)
- Ruby on Rails (3000)
- Django development servers (8000)
- PHP development servers (8000)
- Other web development tools and frameworks

## Icon Design

The status bar icon features:
- **Clean white background** for subtle appearance
- **Color-coded center area** for quick status assessment:
  - Green: 0 processes (safe)
  - Red: 1-9 processes (some development servers)
  - Orange: 10+ processes (many development servers)
- **No borders** for modern, minimal aesthetic

## Security Notes

- Only processes on the specified port range are monitored
- Process termination uses standard Unix signals
- No network communication or data collection
- Requires access to process information and termination capabilities

## Stopping the Application

- Click the status bar icon and select "Quit"
- Or use `pkill -f port-kill` from terminal
- The application will cleanly shut down all monitoring threads
