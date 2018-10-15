using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine;
using UnityEngine.UI;
//Esteban
public class P1Humain : MonoBehaviour {

	public Button p1Humain;

	// Use this for initialization
	void Start () {

		Button btn1 = p1Humain.GetComponent<Button>();
		btn1.onClick.AddListener(TaskOnClick);
	}
	
	// Update is called once per frame
	void Update () {
		
	}
	
	void TaskOnClick()
	{
		Debug.Log("You have clicked the button4!");
		PlayerType.MyPlayersType.typeP1txt.text = "Human";
		PlayerType.MyPlayersType.typeP1 = 1;
	}
}
