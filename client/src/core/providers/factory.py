import configparser
import os
from .base import IATCProvider
from .local import LocalSpeechProvider
from .cloud import SayIntentionsProvider

def get_provider(config_path: str = "config.ini") -> IATCProvider:
    """
    Factory function to return the configured ATC provider.
    """
    config = configparser.ConfigParser()
    # Try multiple paths for config
    paths = [
        config_path,
        os.path.expanduser("~/.config/stratusai/config.ini"),
        "config.ini",
        "../config.ini"
    ]
    
    found_config = False
    for path in paths:
        if os.path.exists(path):
            config.read(path)
            found_config = True
            break
            
    backend = "local"
    if found_config and config.has_option("general", "backend"):
        backend = config.get("general", "backend").lower()
        
    if backend == "cloud" or backend == "sayintentions":
        api_key = config.get("sapi", "api_key", fallback="")
        return SayIntentionsProvider(api_key)
    else:
        return LocalSpeechProvider()
