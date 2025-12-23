from abc import ABC, abstractmethod
import time
import logging

class ISapiService(ABC):
    """Interface for the SayIntentions Cloud API (SAPI)"""

    @abstractmethod
    def connect(self, api_key: str) -> bool:
        """Authenticate with the service"""
        pass

    @abstractmethod
    def send_audio(self, audio_data: bytes) -> str:
        """Stream audio to the brain and return a transaction ID or immediate response status"""
        pass

    @abstractmethod
    def get_status(self) -> str:
        """Get current connection status"""
        pass

class MockSapiService(ISapiService):
    """Mock implementation for development without an API Key"""
    
    def __init__(self):
        self._connected = False
        self.logger = logging.getLogger("MockSapi")

    def connect(self, api_key: str) -> bool:
        self.logger.info(f"Mock connecting with key: {api_key}")
        time.sleep(1) # Simulate network delay
        self._connected = True
        return True

    def send_audio(self, audio_data: bytes) -> str:
        if not self._connected:
            raise ConnectionError("Not connected")
        
        self.logger.info(f"Mock received {len(audio_data)} bytes of audio")
        # In a real scenario, this might return a JSON response or stream audio back.
        # For now, we'll just simulate a successful 'request accepted'
        return "mock_transaction_id_123"

    def get_status(self) -> str:
        return "CONNECTED (MOCK)" if self._connected else "DISCONNECTED"
