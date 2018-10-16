using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
//Esteban
public class P2Back : MonoBehaviour {

	public Button p2Human;

	// Use this for initialization
	void Start () {

		Button btn1 = p2Human.GetComponent<Button>();
		btn1.onClick.AddListener(TaskOnClick);
	}
	
	// Update is called once per frame
	void Update () {
		
	}
	
	void TaskOnClick()
	{
		PlayerType.MyPlayersType.p2id-=1;
		if (PlayerType.MyPlayersType.p2id < 0)
		{
			PlayerType.MyPlayersType.p2id = 4;
		}
		PlayerType.MyPlayersType.NewNamep2(PlayerType.MyPlayersType.p2id);
	}
}
