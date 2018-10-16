﻿using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
//Esteban
public class P1Next : MonoBehaviour {

	public Button p1Random;

	// Use this for initialization
	void Start () {

		Button btn1 = p1Random.GetComponent<Button>();
		btn1.onClick.AddListener(TaskOnClick);
	}
	
	// Update is called once per frame
	void Update () {
		
	}
	
	void TaskOnClick()
	{
		PlayerType.MyPlayersType.p1id+=1;
		if (PlayerType.MyPlayersType.p1id > 4)
		{
			PlayerType.MyPlayersType.p1id = 0;
		}
		PlayerType.MyPlayersType.NewNamep1(PlayerType.MyPlayersType.p1id);
	}
}