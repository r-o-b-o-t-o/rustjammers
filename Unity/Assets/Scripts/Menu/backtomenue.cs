using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.SceneManagement;

public class backtomenue : MonoBehaviour {

	public Button Back;

	// Use this for initialization
	void Start () {
		Button btn1 = Back.GetComponent<Button>();
		btn1.onClick.AddListener(TaskOnClick);
	}
	
	
	void TaskOnClick()
	{
		SceneManager.LoadScene("Menu");
	}
}
