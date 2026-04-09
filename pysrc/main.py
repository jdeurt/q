import requests 
import random
import json
import subprocess
from google import genAI
import platform
import os
#python ~/q/pysrc/main.py

def  send_commend_togemini(api:str, msg:str , model_name):
    os_name = platform.system().lower()
    shell = os.environ.get('SHELL')
    msg_rules = f"""
        "You translate natural language into shell commands. The user's OS is {os_name} and their shell is {shell}.\n\
         \n\
         Rules:\n\
         - Output ONLY the shell command. Nothing else.\n\
         - No explanations, no markdown, no backticks, no commentary, no prefixes.\n\
         - Your entire response will be passed directly to sh -c, so it must be a valid shell command.\n\
         - If the request doesn't map to a shell command, output the closest useful command.\n\
         - If there is truly no relevant command, output: echo \"No applicable command.\"\n\
         - For destructive operations (rm, drop, truncate, etc.), prefer safer variants \
         (e.g. rm -i, trash) unless the user's phrasing clearly indicates they want the forceful version.

         user_message = ({msg})
        """
    client = genAI.Client(api_key= api)
    respone = client.model.generate_content(
        model=model_name,contents=msg_rules
    )

    if respone.text:
        userprem = input(f"ARE YOU SURE YOU WANT TO ALLOW  {model_name} to run This commend {respone.text},[y/n]").lower()
        if userprem == "y":
            commend = respone.text
            subprocess.run(commend , shell=True)


# def send_toantropicclaude()  NOT IMPLEMENTED YET!

if __name__ == "__main__":
    send_commend_togemini("YOUR API HERE","YOUR MESSAGE HERE","MODEL NAME e.g gemini-3-flash-preview")