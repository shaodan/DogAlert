import requests
import time


def GetHistory(address: str):
    params = {
        "user_addr": address
    }
    try:
        resp = requests.get("https://api.debank.com/history/list", params).json()
        if resp["error_code"] == 0:
            return resp["data"]["history_list"]
        print("Error code: ", resp["error_code"])
        return None
    except Exception as ex:
        print(ex)
        return None

def send_tg_message():
    pass

def main():
    last_time = 0
    while True:
        history = GetHistory("0xebcd54d901596ce65fa504d96a397e8463d6262d")
        last_tx = history[0]
        if last_time == 0:
            last_time = last_tx["time_at"]
            continue
        if last_time < last_tx["time_at"]:
            last_time = last_tx["time_at"]

        time.sleep(60)
