using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
//Esteban
public class P2Human : MonoBehaviour {

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
		PlayerType.MyPlayersType.typeP2txt.text = "Human";
		PlayerType.MyPlayersType.typeP2 = 1;
	}
}
