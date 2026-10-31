export class P2PNetworkManager {
    constructor(session, peerId) {
        this.session = session;
        this.peerId = peerId;
        this.pc = null;
        this.dataChannel = null;
        this.isConnected = false;
    }

    initConnection(isInitiator, remoteOfferCallback) {
        const configuration = {
            iceServers: [{ urls: 'stun:stun.l.google.com:19302' }]
        };
        
        this.pc = new RTCPeerConnection(configuration);

        this.pc.onicecandidate = (event) => {
            if (event.candidate) {
                // Send ICE candidate to peer via signaling server / clipboard / QR
                console.log("New ICE Candidate:", JSON.stringify(event.candidate));
            }
        };

        if (isInitiator) {
            this.dataChannel = this.pc.createDataChannel("game_sync", { ordered: false, maxRetransmits: 0 });
            this.setupDataChannel();
            
            this.pc.createOffer().then(offer => {
                this.pc.setLocalDescription(offer);
                remoteOfferCallback(offer);
            });
        } else {
            this.pc.ondatachannel = (event) => {
                this.dataChannel = event.channel;
                this.setupDataChannel();
            };
        }
    }

    setupDataChannel() {
        this.dataChannel.onopen = () => {
            this.isConnected = true;
            console.log("WebRTC Data Channel Open - P2P Sync Active");
        };

        this.dataChannel.onmessage = (event) => {
            const msg = JSON.parse(event.data);
            if (msg.type === 'PlayerInput') {
                // Forward remote peer input into WASM session input queue
                this.session.push_player_input(
                    msg.tick, 
                    msg.aim_angle, 
                    50.0, 
                    msg.move_dir, 
                    msg.fire
                );
            }
        };
    }

    sendInput(tick, moveDir, aimAngle, fire) {
        if (!this.isConnected || this.dataChannel.readyState !== 'open') return;

        const payload = JSON.stringify({
            type: 'PlayerInput',
            peer_id: this.peerId,
            tick,
            move_dir: moveDir,
            aim_angle: aimAngle,
            fire
        });

        this.dataChannel.send(payload);
    }
}
