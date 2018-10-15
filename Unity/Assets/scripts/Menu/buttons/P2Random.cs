using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
//Esteban
public class P2Random : MonoBehaviour {

	public Button p2Random;

	// Use this for initialization
	void Start () {

		Button btn1 = p2Random.GetComponent<Button>();
		btn1.onClick.AddListener(TaskOnClick);
	}
	
	// Update is called once per frame
	void Update () {
		
	}
	
	void TaskOnClick()
	{
		Debug.Log("You have clicked the button3!");
		PlayerType.MyPlayersType.typeP2txt.text = "Random";
		PlayerType.MyPlayersType.typeP2 = 0;
	}
}
